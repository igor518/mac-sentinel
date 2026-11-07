use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
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
	pub storage_status: StorageStatus,
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
			storage_status: StorageStatus::Unavailable,
			processes: Vec::new(),
			alerts: Vec::new(),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertEvidence {
	pub id: String,
	pub title: String,
	pub summary: String,
}
