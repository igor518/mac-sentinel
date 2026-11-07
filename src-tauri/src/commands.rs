use tauri::State;

use crate::model::MonitoringSnapshot;
use crate::monitoring::MonitoringState;

#[tauri::command]
pub fn get_current_snapshot(
	monitoring: State<'_, MonitoringState>,
) -> Result<MonitoringSnapshot, String> {
	Ok(monitoring.current())
}
