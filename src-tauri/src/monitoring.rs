use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sysinfo::{ProcessesToUpdate, System};
use tauri::{AppHandle, Emitter};

use crate::model::{MonitoringSnapshot, ProcessKey, ProcessSnapshot, StorageStatus};

pub const DEFAULT_CADENCE_MS: u64 = 2_000;
pub const MONITORING_EVENT: &str = "monitoring://snapshot";

#[derive(Clone, Debug)]
pub struct MonitoringState {
	current: Arc<RwLock<MonitoringSnapshot>>,
}

impl MonitoringState {
	pub fn unavailable() -> Self {
		Self {
			current: Arc::new(RwLock::new(MonitoringSnapshot::unavailable())),
		}
	}

	pub fn current(&self) -> MonitoringSnapshot {
		self.current
			.read()
			.map(|snapshot| snapshot.clone())
			.unwrap_or_else(|_| MonitoringSnapshot::unavailable())
	}

	fn replace(&self, snapshot: MonitoringSnapshot) {
		if let Ok(mut current) = self.current.write() {
			*current = snapshot;
		}
	}
}

pub fn normalize_process_cpu(value: f32) -> Option<f32> {
	value.is_finite().then(|| value.max(0.0))
}

pub fn normalize_system_cpu(value: f32) -> Option<f32> {
	value.is_finite().then(|| value.clamp(0.0, 100.0))
}

pub const fn is_sampling_gap(previous_ms: u64, current_ms: u64, cadence_ms: u64) -> bool {
	current_ms.saturating_sub(previous_ms) > cadence_ms.saturating_mul(2)
}

fn now_ms() -> u64 {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap_or_default()
		.as_millis()
		.try_into()
		.unwrap_or(u64::MAX)
}

fn collect(system: &System, previous_ms: Option<u64>) -> MonitoringSnapshot {
	let recorded_at_ms = now_ms();
	let mut processes = system.processes()
		.iter()
		.map(|(pid, process)| ProcessSnapshot {
			key: ProcessKey::new(pid.as_u32(), process.start_time()),
			name: process.name().to_string_lossy().into_owned(),
			executable_path: process.exe().map(|path| path.to_string_lossy().into_owned()),
			parent_pid: process.parent().map(|parent| parent.as_u32()),
			cpu_percent: normalize_process_cpu(process.cpu_usage()),
			memory_bytes: process.memory(),
		})
		.collect::<Vec<_>>();

	processes.sort_by(|left, right| {
		right.cpu_percent
			.partial_cmp(&left.cpu_percent)
			.unwrap_or(std::cmp::Ordering::Equal)
	});

	MonitoringSnapshot {
		recorded_at_ms,
		system_cpu_percent: normalize_system_cpu(system.global_cpu_usage()),
		used_memory_bytes: Some(system.used_memory()),
		process_count: Some(processes.len()),
		sampling_gap: previous_ms
			.is_some_and(|previous| is_sampling_gap(previous, recorded_at_ms, DEFAULT_CADENCE_MS)),
		storage_status: StorageStatus::Unavailable,
		processes,
		alerts: Vec::new(),
	}
}

pub fn start_monitoring(state: MonitoringState, app: AppHandle) {
	thread::Builder::new()
		.name("macsentinel-monitoring".to_owned())
		.spawn(move || {
			let cadence = Duration::from_millis(DEFAULT_CADENCE_MS);
			let Ok(mut system) = System::new_all() else {
				return;
			};
			let mut previous_ms = None;

			loop {
				thread::sleep(cadence);
				system.refresh_cpu_usage();
				system.refresh_memory();
				system.refresh_processes(ProcessesToUpdate::All, true);

				let snapshot = collect(&system, previous_ms);
				previous_ms = Some(snapshot.recorded_at_ms);
				state.replace(snapshot.clone());
				let _ = app.emit(MONITORING_EVENT, snapshot);
			}
		})
		.expect("failed to start monitoring owner");
}
