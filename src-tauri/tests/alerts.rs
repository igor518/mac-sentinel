use macsentinel_core::alerts::{CpuAlertRule, Evaluation, SamplingContinuity};
use macsentinel_core::model::ProcessKey;

#[test]
fn sustained_cpu_alert_fires_once_after_continuous_duration() {
	let key = ProcessKey::new(42, 1_000);
	let mut rule = CpuAlertRule::new(80.0, 600_000, 300_000).unwrap();

	assert_eq!(rule.evaluate(key, 0, Some(90.0), SamplingContinuity::Continuous), Evaluation::Pending);
	assert_eq!(rule.evaluate(key, 599_999, Some(90.0), SamplingContinuity::Continuous), Evaluation::Pending);
	assert!(matches!(rule.evaluate(key, 600_000, Some(90.0), SamplingContinuity::Continuous), Evaluation::Triggered(_)));
	assert_eq!(rule.evaluate(key, 601_000, Some(90.0), SamplingContinuity::Continuous), Evaluation::Cooldown);
}

#[test]
fn sampling_gap_resets_sustained_cpu_continuity() {
	let key = ProcessKey::new(42, 1_000);
	let mut rule = CpuAlertRule::new(80.0, 600_000, 300_000).unwrap();

	rule.evaluate(key, 0, Some(90.0), SamplingContinuity::Continuous);
	assert_eq!(rule.evaluate(key, 599_999, Some(90.0), SamplingContinuity::Gap), Evaluation::Reset);
	assert_eq!(rule.evaluate(key, 600_000, Some(90.0), SamplingContinuity::Continuous), Evaluation::Pending);
}

#[test]
fn a_reused_pid_does_not_inherit_alert_state() {
	let mut rule = CpuAlertRule::new(80.0, 10_000, 30_000).unwrap();
	rule.evaluate(ProcessKey::new(42, 1_000), 0, Some(90.0), SamplingContinuity::Continuous);

	assert_eq!(rule.evaluate(ProcessKey::new(42, 2_000), 10_000, Some(90.0), SamplingContinuity::Continuous), Evaluation::Pending);
}
