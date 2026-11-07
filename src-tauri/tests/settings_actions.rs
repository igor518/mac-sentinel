use macsentinel_core::model::ProcessKey;
use macsentinel_core::settings::{Settings, SettingsError};
use macsentinel_core::termination::{ProcessAccess, TerminationOutcome, TerminationService};

#[test]
fn default_settings_match_the_approved_contract() {
	let settings = Settings::default();

	assert_eq!(settings.sampling_interval_ms, 2_000);
	assert_eq!(settings.cpu_threshold_percent, 80.0);
	assert_eq!(settings.cpu_sustained_ms, 600_000);
	assert_eq!(settings.raw_retention_ms, 86_400_000);
	assert_eq!(settings.aggregate_retention_ms, 604_800_000);
}

#[test]
fn aggregate_retention_cannot_be_shorter_than_raw_retention() {
	let settings = Settings {
		raw_retention_ms: 604_800_000,
		aggregate_retention_ms: 86_400_000,
		..Settings::default()
	};

	assert_eq!(settings.validate(), Err(SettingsError::AggregateRetentionTooShort));
}

#[test]
fn changed_identity_aborts_termination_before_signal() {
	let access = ProcessAccess::fake(ProcessKey::new(99, 2_000));
	let service = TerminationService::new(access.clone());

	let result = service.terminate_confirmed(ProcessKey::new(99, 1_000));

	assert_eq!(result, TerminationOutcome::IdentityChanged);
	assert_eq!(access.signals_sent(), 0);
}
