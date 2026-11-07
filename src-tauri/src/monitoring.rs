use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, RwLock};
#[cfg(feature = "e2e")]
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sysinfo::{ProcessesToUpdate, System};
use tauri::{AppHandle, Emitter};

use crate::alerts::{CpuAlertRule, Evaluation, SamplingContinuity};
use crate::model::{
	AlertEvidence, MonitoringSnapshot, ProcessKey, ProcessSample, ProcessSnapshot, StorageStatus,
};
use crate::storage::StorageHandle;

pub const DEFAULT_CADENCE_MS: u64 = 2_000;
pub const MONITORING_EVENT: &str = "monitoring://snapshot";
const MAX_ALERT_HISTORY: usize = 100;

#[derive(Clone, Debug)]
pub struct MonitoringState {
	current: Arc<RwLock<MonitoringSnapshot>>,
	#[cfg(feature = "e2e")]
	forced_sampling_gap: Arc<AtomicBool>,
}

impl MonitoringState {
	pub fn unavailable() -> Self {
		Self {
			current: Arc::new(RwLock::new(MonitoringSnapshot::unavailable())),
			#[cfg(feature = "e2e")]
			forced_sampling_gap: Arc::new(AtomicBool::new(false)),
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

	#[cfg(feature = "e2e")]
	pub fn force_sampling_gap(&self) {
		self.forced_sampling_gap.store(true, Ordering::Release);
	}

	#[cfg(feature = "e2e")]
	fn take_forced_sampling_gap(&self) -> bool {
		self.forced_sampling_gap.swap(false, Ordering::AcqRel)
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

fn collect(
	system: &System,
	previous_ms: Option<u64>,
	alert_rule: &mut CpuAlertRule,
	alert_history: &mut VecDeque<AlertEvidence>,
	storage_status: StorageStatus,
	forced_sampling_gap: bool,
) -> MonitoringSnapshot {
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
	let sampling_gap = forced_sampling_gap || previous_ms
		.is_some_and(|previous| is_sampling_gap(previous, recorded_at_ms, DEFAULT_CADENCE_MS));
	let active_keys = processes
		.iter()
		.map(|process| process.key.clone())
		.collect::<HashSet<_>>();
	alert_rule.retain_active(&active_keys);

	if sampling_gap {
		alert_rule.reset_continuity();
	} else {
		for process in &processes {
			if let Evaluation::Triggered(mut evidence) = alert_rule.evaluate(
				process.key.clone(),
				recorded_at_ms,
				process.cpu_percent,
				SamplingContinuity::Continuous,
			) {
				evidence.process_name.clone_from(&process.name);
				evidence.summary = AlertEvidence::sustained_cpu(
					process.key.clone(),
					process.name.clone(),
					evidence.observed_cpu_percent,
					evidence.threshold_cpu_percent,
					evidence.sustained_duration_ms,
					evidence.episode_started_at_ms,
					evidence.triggered_at_ms,
				).summary;
				alert_history.push_front(evidence);
				alert_history.truncate(MAX_ALERT_HISTORY);
			}
		}
	}

	#[cfg(feature = "e2e")]
	if alert_history.is_empty() {
		alert_history.push_front(e2e_alert_fixture(recorded_at_ms));
	}

	MonitoringSnapshot {
		recorded_at_ms,
		system_cpu_percent: normalize_system_cpu(system.global_cpu_usage()),
		used_memory_bytes: Some(system.used_memory()),
		process_count: Some(processes.len()),
		sampling_gap,
		persistence_gap: false,
		storage_status,
		notifications_enabled: None,
		processes,
		alerts: alert_history.iter().cloned().collect(),
	}
}

#[cfg(feature = "e2e")]
fn e2e_alert_fixture(recorded_at_ms: u64) -> AlertEvidence {
	AlertEvidence::sustained_cpu(
		ProcessKey::new(9_999_999, 1),
		"E2E sustained CPU fixture".to_owned(),
		91.0,
		80.0,
		600_000,
		recorded_at_ms.saturating_sub(600_000),
		recorded_at_ms,
	)
}

pub fn start_monitoring(state: MonitoringState, storage: StorageHandle, app: AppHandle) {
	thread::Builder::new()
		.name("macsentinel-monitoring".to_owned())
		.spawn(move || {
			let cadence = Duration::from_millis(DEFAULT_CADENCE_MS);
			let Ok(mut system) = System::new_all() else {
				return;
			};
			let mut previous_ms = None;
			let mut alert_rule = CpuAlertRule::default();
			let mut alert_history = VecDeque::new();

			loop {
				thread::sleep(cadence);
				system.refresh_cpu_usage();
				system.refresh_memory();
				system.refresh_processes(ProcessesToUpdate::All, true);

				#[cfg(feature = "e2e")]
				let forced_sampling_gap = state.take_forced_sampling_gap();
				#[cfg(not(feature = "e2e"))]
				let forced_sampling_gap = false;
				let mut snapshot = collect(
					&system,
					previous_ms,
					&mut alert_rule,
					&mut alert_history,
					storage.status(),
					forced_sampling_gap,
				);
				let samples = snapshot.processes.iter().map(|process| ProcessSample::new(
					process.key,
					snapshot.recorded_at_ms,
					process.cpu_percent,
					process.memory_bytes,
				)).collect();
				if storage.try_append(snapshot.recorded_at_ms, samples).is_err() {
					snapshot.storage_status = storage.status();
					snapshot.persistence_gap = true;
				}
				previous_ms = Some(snapshot.recorded_at_ms);
				state.replace(snapshot.clone());
				let _ = app.emit(MONITORING_EVENT, snapshot);
			}
		})
		.expect("failed to start monitoring owner");
}
