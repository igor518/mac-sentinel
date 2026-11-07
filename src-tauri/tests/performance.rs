use std::time::Duration;

use macsentinel_core::performance::measure_idle_resource_use;

#[test]
#[ignore = "run on the approved MacBook Pro reference machine"]
fn idle_resource_use_stays_within_the_approved_budget() {
	let measurement = measure_idle_resource_use(Duration::from_secs(60)).unwrap();

	assert!(measurement.average_cpu_percent < 1.0);
	assert!(measurement.maximum_resident_memory_bytes < 100 * 1024 * 1024);
}
