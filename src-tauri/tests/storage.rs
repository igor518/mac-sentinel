use macsentinel_core::model::{ProcessKey, ProcessSample};
use macsentinel_core::storage::{HistoryStore, Retention};

#[test]
fn retention_prunes_raw_samples_and_preserves_recent_data() {
	let directory = tempfile::tempdir().unwrap();
	let mut store = HistoryStore::open(directory.path().join("history.sqlite")).unwrap();
	let key = ProcessKey::new(7, 100);

	store.append_process_samples(&[
		ProcessSample::new(key, 1_000, Some(10.0), 100),
		ProcessSample::new(key, 10_000, Some(20.0), 200),
	]).unwrap();
	store.prune(20_000, Retention { raw_ms: 15_000, aggregate_ms: 30_000 }).unwrap();

	let samples = store.process_history(key, 0, 20_000).unwrap();
	assert_eq!(samples.len(), 1);
	assert_eq!(samples[0].recorded_at_ms, 10_000);
}

#[test]
fn process_history_isolated_by_pid_and_start_time() {
	let directory = tempfile::tempdir().unwrap();
	let mut store = HistoryStore::open(directory.path().join("history.sqlite")).unwrap();
	let original = ProcessKey::new(7, 100);
	let reused = ProcessKey::new(7, 200);

	store.append_process_samples(&[
		ProcessSample::new(original, 1_000, Some(10.0), 100),
		ProcessSample::new(reused, 2_000, Some(20.0), 200),
	]).unwrap();

	assert_eq!(store.process_history(original, 0, 3_000).unwrap().len(), 1);
	assert_eq!(store.process_history(reused, 0, 3_000).unwrap().len(), 1);
}
