pub mod commands;
pub mod model;
pub mod monitoring;
pub mod platform;

use monitoring::{MonitoringState, start_monitoring};
use tauri::Manager;

pub fn run() {
	let monitoring = MonitoringState::unavailable();
	let monitoring_owner = monitoring.clone();

	let builder = tauri::Builder::default();

	#[cfg(feature = "e2e")]
	let builder = builder
		.plugin(tauri_plugin_wdio::init())
		.plugin(tauri_plugin_wdio_webdriver::init());

	builder
		.manage(monitoring)
		.invoke_handler(tauri::generate_handler![commands::get_current_snapshot])
		.setup(move |app| {
			start_monitoring(monitoring_owner.clone(), app.handle().clone());

			if let Some(window) = app.get_webview_window("main") {
				window.show()?;
				window.set_focus()?;
			}

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("failed to run MacSentinel");
}
