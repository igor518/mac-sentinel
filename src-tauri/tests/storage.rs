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

#[test]
fn storage_uses_versioned_wal_schema() {
	let directory = tempfile::tempdir().unwrap();
	let store = HistoryStore::open(directory.path().join("history.sqlite")).unwrap();

	assert_eq!(store.schema_version().unwrap(), 2);
	assert_eq!(store.journal_mode().unwrap().as_deref(), Some("wal"));
}

#[test]
fn an_unopenable_database_is_reported_without_recreation() {
	let directory = tempfile::tempdir().unwrap();
	let result = HistoryStore::open(directory.path());

	assert!(result.is_err());
	assert!(directory.path().is_dir());
}

#[test]
fn aggregate_buckets_outlive_raw_samples_within_aggregate_retention() {
	let directory = tempfile::tempdir().unwrap();
	let mut store = HistoryStore::open(directory.path().join("history.sqlite")).unwrap();
	let key = ProcessKey::new(7, 100);
	store.append_process_samples(&[
		ProcessSample::new(key, 60_000, Some(10.0), 100),
		ProcessSample::new(key, 90_000, Some(30.0), 300),
	]).unwrap();

	store.prune(200_000, Retention { raw_ms: 50_000, aggregate_ms: 180_000 }).unwrap();

	assert!(store.process_history(key, 0, 200_000).unwrap().is_empty());
	assert_eq!(store.aggregate_bucket_count(key, 0, 200_000).unwrap(), 1);
}
