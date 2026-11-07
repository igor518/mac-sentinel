use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessKey {
	pub pid: u32,
	pub started_at: u64,
}

impl ProcessKey {
	pub const fn new(pid: u32, started_at: u64) -> Self {
		Self { pid, started_at }
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessSnapshot {
	pub key: ProcessKey,
	pub name: String,
	pub executable_path: Option<String>,
	pub parent_pid: Option<u32>,
	pub cpu_percent: Option<f32>,
	pub memory_bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessSample {
	pub key: ProcessKey,
	pub recorded_at_ms: u64,
	pub cpu_percent: Option<f32>,
	pub memory_bytes: u64,
}

impl ProcessSample {
	pub const fn new(
		key: ProcessKey,
		recorded_at_ms: u64,
		cpu_percent: Option<f32>,
		memory_bytes: u64,
	) -> Self {
		Self {
			key,
			recorded_at_ms,
			cpu_percent,
			memory_bytes,
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StorageStatus {
	Available,
	Degraded,
	Unavailable,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringSnapshot {
	pub recorded_at_ms: u64,
	pub system_cpu_percent: Option<f32>,
	pub used_memory_bytes: Option<u64>,
	pub process_count: Option<usize>,
	pub sampling_gap: bool,
	pub persistence_gap: bool,
	pub storage_status: StorageStatus,
	pub notifications_enabled: Option<bool>,
	pub processes: Vec<ProcessSnapshot>,
	pub alerts: Vec<AlertEvidence>,
}

impl MonitoringSnapshot {
	pub fn unavailable() -> Self {
		Self {
			recorded_at_ms: 0,
			system_cpu_percent: None,
			used_memory_bytes: None,
			process_count: None,
			sampling_gap: false,
			persistence_gap: false,
			storage_status: StorageStatus::Unavailable,
			notifications_enabled: None,
			processes: Vec::new(),
			alerts: Vec::new(),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertEvidence {
	pub id: String,
	pub process_key: ProcessKey,
	pub process_name: String,
	pub observed_cpu_percent: f32,
	pub threshold_cpu_percent: f32,
	pub sustained_duration_ms: u64,
	pub episode_started_at_ms: u64,
	pub triggered_at_ms: u64,
	pub title: String,
	pub summary: String,
}

impl AlertEvidence {
	pub fn sustained_cpu(
		process_key: ProcessKey,
		process_name: String,
		observed_cpu_percent: f32,
		threshold_cpu_percent: f32,
		sustained_duration_ms: u64,
		episode_started_at_ms: u64,
		triggered_at_ms: u64,
	) -> Self {
		let duration_minutes = if sustained_duration_ms % 60_000 == 0 {
			(sustained_duration_ms / 60_000).to_string()
		} else {
			format!("{:.1}", sustained_duration_ms as f64 / 60_000.0)
		};
		let title = "Sustained CPU usage".to_owned();
		let summary = format!(
			"{process_name} reached {observed_cpu_percent:.0}% CPU against a {threshold_cpu_percent:.0}% threshold for a {duration_minutes} minute duration."
		);
		let id = format!(
			"cpu:{}:{}:{episode_started_at_ms}",
			process_key.pid, process_key.started_at
		);

		Self {
			id,
			process_key,
			process_name,
			observed_cpu_percent,
			threshold_cpu_percent,
			sustained_duration_ms,
			episode_started_at_ms,
			triggered_at_ms,
			title,
			summary,
		}
	}
}
