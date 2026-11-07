use std::fs;
use std::path::PathBuf;

fn project_root() -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

#[test]
fn production_capabilities_do_not_grant_http_access() {
	let capabilities = fs::read_to_string(
		project_root().join("src-tauri/capabilities/default.json"),
	).unwrap();

	assert!(!capabilities.contains("http:"));
	assert!(!capabilities.contains("shell:allow-execute"));
}

#[test]
fn bundle_identifier_and_license_match_the_approved_contract() {
	let tauri_config = fs::read_to_string(project_root().join("src-tauri/tauri.conf.json")).unwrap();
	let license = fs::read_to_string(project_root().join("LICENSE")).unwrap();

	assert!(tauri_config.contains("com.macsentinel.app"));
	assert!(license.contains("Apache License"));
	assert!(license.contains("Version 2.0, January 2004"));
}
