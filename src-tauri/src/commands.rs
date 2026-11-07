use tauri::State;

use crate::model::{MonitoringSnapshot, ProcessKey, ProcessSample};
use crate::monitoring::MonitoringState;
use crate::storage::StorageHandle;

#[tauri::command]
pub fn get_current_snapshot(
	monitoring: State<'_, MonitoringState>,
) -> Result<MonitoringSnapshot, String> {
	Ok(monitoring.current())
}

#[tauri::command]
pub fn get_process_history(
	storage: State<'_, StorageHandle>,
	key: ProcessKey,
	from_ms: u64,
	to_ms: u64,
) -> Result<Vec<ProcessSample>, String> {
	storage
		.process_history(key, from_ms, to_ms)
		.map_err(|error| error.to_string())
}

#[cfg(feature = "e2e")]
#[tauri::command]
pub fn simulate_storage_failure(storage: State<'_, StorageHandle>) -> Result<(), String> {
	storage.disable_for_e2e().map_err(|error| error.to_string())
}

#[cfg(feature = "e2e")]
#[tauri::command]
pub fn simulate_sampling_gap(monitoring: State<'_, MonitoringState>) {
	monitoring.force_sampling_gap();
}
