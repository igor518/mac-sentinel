use macsentinel_core::model::ProcessKey;
use macsentinel_core::monitoring::{is_sampling_gap, normalize_process_cpu, normalize_system_cpu};

#[test]
fn pid_reuse_produces_distinct_process_keys() {
	let original = ProcessKey::new(4242, 100);
	let reused = ProcessKey::new(4242, 101);

	assert_ne!(original, reused);
}

#[test]
fn process_cpu_can_exceed_one_hundred_percent() {
	assert_eq!(normalize_process_cpu(241.5), Some(241.5));
}

#[test]
fn system_cpu_is_bounded_to_total_machine_scale() {
	assert_eq!(normalize_system_cpu(-1.0), Some(0.0));
	assert_eq!(normalize_system_cpu(52.5), Some(52.5));
	assert_eq!(normalize_system_cpu(120.0), Some(100.0));
}

#[test]
fn unavailable_cpu_is_not_fabricated_as_zero() {
	assert_eq!(normalize_process_cpu(f32::NAN), None);
}

#[test]
fn interval_over_twice_the_cadence_is_a_gap() {
	assert!(!is_sampling_gap(1_000, 5_000, 2_000));
	assert!(is_sampling_gap(1_000, 5_001, 2_000));
}
