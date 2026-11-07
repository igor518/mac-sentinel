pub mod alerts;
pub mod commands;
pub mod model;
pub mod monitoring;
pub mod platform;
pub mod storage;

use monitoring::{MonitoringState, start_monitoring};
use storage::StorageHandle;
use tauri::Manager;

pub fn run() {
	let monitoring = MonitoringState::unavailable();
	let monitoring_owner = monitoring.clone();

	let builder = tauri::Builder::default();

	#[cfg(feature = "e2e")]
	let builder = builder
		.plugin(tauri_plugin_wdio::init())
		.plugin(tauri_plugin_wdio_webdriver::init());

	#[cfg(feature = "e2e")]
	let builder = builder.invoke_handler(tauri::generate_handler![
		commands::get_current_snapshot,
		commands::get_process_history,
		commands::simulate_storage_failure,
		commands::simulate_sampling_gap,
	]);
	#[cfg(not(feature = "e2e"))]
	let builder = builder.invoke_handler(tauri::generate_handler![
		commands::get_current_snapshot,
		commands::get_process_history,
	]);

	builder
		.manage(monitoring)
		.setup(move |app| {
			let storage_path = app.path().app_data_dir()?.join("history.sqlite");
			let storage = StorageHandle::start(storage_path);
			app.manage(storage.clone());
			start_monitoring(monitoring_owner.clone(), storage, app.handle().clone());

			if let Some(window) = app.get_webview_window("main") {
				window.show()?;
				window.set_focus()?;
			}

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("failed to run MacSentinel");
}
