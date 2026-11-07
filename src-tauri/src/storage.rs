use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, SyncSender, TrySendError};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, OptionalExtension, params};
use thiserror::Error;

use crate::model::{ProcessKey, ProcessSample, StorageStatus};

const SCHEMA_VERSION: i64 = 2;
const AGGREGATE_BUCKET_MS: u64 = 60_000;
const STORAGE_QUEUE_CAPACITY: usize = 8;
const STORAGE_REPLY_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Retention {
	pub raw_ms: u64,
	pub aggregate_ms: u64,
}

impl Default for Retention {
	fn default() -> Self {
		Self {
			raw_ms: 24 * 60 * 60 * 1_000,
			aggregate_ms: 7 * 24 * 60 * 60 * 1_000,
		}
	}
}

#[derive(Debug, Error)]
pub enum StorageError {
	#[error("history storage is unavailable")]
	Unavailable,
	#[error("history storage did not respond")]
	Timeout,
	#[error("history storage queue is full")]
	QueueFull,
	#[error(transparent)]
	Database(#[from] rusqlite::Error),
}

pub struct HistoryStore {
	connection: Connection,
}

impl HistoryStore {
	pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
		let path = path.as_ref();
		if let Some(parent) = path.parent() {
			std::fs::create_dir_all(parent).map_err(|_| StorageError::Unavailable)?;
		}

		let connection = Connection::open(path)?;
		connection.busy_timeout(Duration::from_secs(2))?;
		connection.pragma_update(None, "foreign_keys", true)?;
		connection.pragma_update(None, "journal_mode", "WAL")?;
		let mut store = Self { connection };
		store.migrate()?;
		Ok(store)
	}

	fn migrate(&mut self) -> Result<(), StorageError> {
		let current_version = self.connection.pragma_query_value(
			None,
			"user_version",
			|row| row.get::<_, i64>(0),
		)?;

		if current_version > SCHEMA_VERSION {
			return Err(StorageError::Unavailable);
		}

		if current_version == 0 {
			let transaction = self.connection.transaction()?;
			transaction.execute_batch(
				"CREATE TABLE process_samples (
					pid INTEGER NOT NULL,
					started_at INTEGER NOT NULL,
					recorded_at_ms INTEGER NOT NULL,
					cpu_percent REAL,
					memory_bytes INTEGER NOT NULL,
					PRIMARY KEY (pid, started_at, recorded_at_ms)
				);
				CREATE INDEX process_samples_recorded_at
					ON process_samples(recorded_at_ms);
				CREATE TABLE process_aggregates (
					pid INTEGER NOT NULL,
					started_at INTEGER NOT NULL,
					bucket_started_at_ms INTEGER NOT NULL,
					average_cpu_percent REAL,
					average_memory_bytes INTEGER NOT NULL,
					sample_count INTEGER NOT NULL,
					PRIMARY KEY (pid, started_at, bucket_started_at_ms)
				);
				CREATE INDEX process_aggregates_recorded_at
					ON process_aggregates(bucket_started_at_ms);",
			)?;
			transaction.pragma_update(None, "user_version", SCHEMA_VERSION)?;
			transaction.commit()?;
		} else if current_version == 1 {
			let transaction = self.connection.transaction()?;
			transaction.execute(
				"ALTER TABLE process_aggregates
				ADD COLUMN sample_count INTEGER NOT NULL DEFAULT 1",
				[],
			)?;
			transaction.pragma_update(None, "user_version", SCHEMA_VERSION)?;
			transaction.commit()?;
		}

		Ok(())
	}

	pub fn append_process_samples(&mut self, samples: &[ProcessSample]) -> Result<(), StorageError> {
		if samples.is_empty() {
			return Ok(());
		}

		let transaction = self.connection.transaction()?;
		{
			let mut statement = transaction.prepare_cached(
				"INSERT OR REPLACE INTO process_samples
				(pid, started_at, recorded_at_ms, cpu_percent, memory_bytes)
				VALUES (?1, ?2, ?3, ?4, ?5)",
			)?;
			for sample in samples {
				statement.execute(params![
					i64::from(sample.key.pid),
					u64_to_i64(sample.key.started_at)?,
					u64_to_i64(sample.recorded_at_ms)?,
					sample.cpu_percent,
					u64_to_i64(sample.memory_bytes)?,
				])?;
			}
		}
		{
			let mut statement = transaction.prepare_cached(
				"INSERT INTO process_aggregates
				(pid, started_at, bucket_started_at_ms, average_cpu_percent,
				average_memory_bytes, sample_count)
				VALUES (?1, ?2, ?3, ?4, ?5, 1)
				ON CONFLICT(pid, started_at, bucket_started_at_ms) DO UPDATE SET
					average_cpu_percent = CASE
						WHEN excluded.average_cpu_percent IS NULL THEN average_cpu_percent
						WHEN average_cpu_percent IS NULL THEN excluded.average_cpu_percent
						ELSE ((average_cpu_percent * sample_count)
							+ excluded.average_cpu_percent) / (sample_count + 1)
					END,
					average_memory_bytes = ((average_memory_bytes * sample_count)
						+ excluded.average_memory_bytes) / (sample_count + 1),
					sample_count = sample_count + 1",
			)?;
			for sample in samples {
				let bucket_started_at_ms = sample.recorded_at_ms
					.saturating_sub(sample.recorded_at_ms % AGGREGATE_BUCKET_MS);
				statement.execute(params![
					i64::from(sample.key.pid),
					u64_to_i64(sample.key.started_at)?,
					u64_to_i64(bucket_started_at_ms)?,
					sample.cpu_percent,
					u64_to_i64(sample.memory_bytes)?,
				])?;
			}
		}
		transaction.commit()?;
		Ok(())
	}

	pub fn process_history(
		&self,
		key: ProcessKey,
		from_ms: u64,
		to_ms: u64,
	) -> Result<Vec<ProcessSample>, StorageError> {
		let mut statement = self.connection.prepare_cached(
			"SELECT recorded_at_ms, cpu_percent, memory_bytes
			FROM process_samples
			WHERE pid = ?1 AND started_at = ?2
				AND recorded_at_ms BETWEEN ?3 AND ?4
			ORDER BY recorded_at_ms ASC",
		)?;
		let rows = statement.query_map(
			params![
				i64::from(key.pid),
				u64_to_i64(key.started_at)?,
				u64_to_i64(from_ms)?,
				u64_to_i64(to_ms)?,
			],
			|row| {
				Ok(ProcessSample::new(
					key,
					row.get::<_, i64>(0)?.try_into().unwrap_or_default(),
					row.get(1)?,
					row.get::<_, i64>(2)?.try_into().unwrap_or_default(),
				))
			},
		)?;

		rows.collect::<Result<Vec<_>, _>>().map_err(StorageError::from)
	}

	pub fn prune(&mut self, now_ms: u64, retention: Retention) -> Result<(), StorageError> {
		let raw_cutoff = now_ms.saturating_sub(retention.raw_ms);
		let aggregate_cutoff = now_ms.saturating_sub(retention.aggregate_ms);
		let transaction = self.connection.transaction()?;
		transaction.execute(
			"DELETE FROM process_samples WHERE recorded_at_ms < ?1",
			params![u64_to_i64(raw_cutoff)?],
		)?;
		transaction.execute(
			"DELETE FROM process_aggregates WHERE bucket_started_at_ms < ?1",
			params![u64_to_i64(aggregate_cutoff)?],
		)?;
		transaction.commit()?;
		Ok(())
	}

	pub fn schema_version(&self) -> Result<i64, StorageError> {
		self.connection
			.pragma_query_value(None, "user_version", |row| row.get(0))
			.map_err(StorageError::from)
	}

	pub fn aggregate_bucket_count(
		&self,
		key: ProcessKey,
		from_ms: u64,
		to_ms: u64,
	) -> Result<usize, StorageError> {
		let count = self.connection.query_row(
			"SELECT COUNT(*) FROM process_aggregates
			WHERE pid = ?1 AND started_at = ?2
				AND bucket_started_at_ms BETWEEN ?3 AND ?4",
			params![
				i64::from(key.pid),
				u64_to_i64(key.started_at)?,
				u64_to_i64(from_ms)?,
				u64_to_i64(to_ms)?,
			],
			|row| row.get::<_, i64>(0),
		).map_err(StorageError::from)?;
		count.try_into().map_err(|_| StorageError::Unavailable)
	}

	pub fn journal_mode(&self) -> Result<Option<String>, StorageError> {
		self.connection
			.query_row("PRAGMA journal_mode", [], |row| row.get(0))
			.optional()
			.map_err(StorageError::from)
	}
}

fn u64_to_i64(value: u64) -> Result<i64, StorageError> {
	value.try_into().map_err(|_| StorageError::Unavailable)
}

enum StorageCommand {
	Append {
		recorded_at_ms: u64,
		samples: Vec<ProcessSample>,
	},
	#[cfg(feature = "e2e")]
	DisableForE2e,
	History {
		key: ProcessKey,
		from_ms: u64,
		to_ms: u64,
		reply: mpsc::Sender<Result<Vec<ProcessSample>, StorageError>>,
	},
}

#[derive(Clone, Debug)]
pub struct StorageHandle {
	sender: SyncSender<StorageCommand>,
	status: Arc<RwLock<StorageStatus>>,
}

impl StorageHandle {
	pub fn start(path: PathBuf) -> Self {
		let (sender, receiver) = mpsc::sync_channel(STORAGE_QUEUE_CAPACITY);
		let status = Arc::new(RwLock::new(StorageStatus::Degraded));
		let worker_status = status.clone();
		thread::Builder::new()
			.name("macsentinel-storage".to_owned())
			.spawn(move || run_storage_worker(path, receiver, worker_status))
			.expect("failed to start history storage owner");

		Self { sender, status }
	}

	pub fn status(&self) -> StorageStatus {
		self.status.read().map(|status| status.clone()).unwrap_or(StorageStatus::Unavailable)
	}

	pub fn try_append(
		&self,
		recorded_at_ms: u64,
		samples: Vec<ProcessSample>,
	) -> Result<(), StorageError> {
		match self.sender.try_send(StorageCommand::Append { recorded_at_ms, samples }) {
			Ok(()) => Ok(()),
			Err(TrySendError::Full(_)) => {
				set_status(&self.status, StorageStatus::Degraded);
				Err(StorageError::QueueFull)
			}
			Err(TrySendError::Disconnected(_)) => {
				set_status(&self.status, StorageStatus::Unavailable);
				Err(StorageError::Unavailable)
			}
		}
	}

	pub fn process_history(
		&self,
		key: ProcessKey,
		from_ms: u64,
		to_ms: u64,
	) -> Result<Vec<ProcessSample>, StorageError> {
		let (reply, receiver) = mpsc::channel();
		self.sender
			.try_send(StorageCommand::History { key, from_ms, to_ms, reply })
			.map_err(|error| match error {
				TrySendError::Full(_) => StorageError::QueueFull,
				TrySendError::Disconnected(_) => StorageError::Unavailable,
			})?;
		receiver.recv_timeout(STORAGE_REPLY_TIMEOUT).map_err(|_| StorageError::Timeout)?
	}

	#[cfg(feature = "e2e")]
	pub fn disable_for_e2e(&self) -> Result<(), StorageError> {
		self.sender
			.try_send(StorageCommand::DisableForE2e)
			.map_err(|error| match error {
				TrySendError::Full(_) => StorageError::QueueFull,
				TrySendError::Disconnected(_) => StorageError::Unavailable,
			})
	}
}

fn run_storage_worker(
	path: PathBuf,
	receiver: Receiver<StorageCommand>,
	status: Arc<RwLock<StorageStatus>>,
) {
	let Ok(mut store) = HistoryStore::open(path) else {
		set_status(&status, StorageStatus::Unavailable);
		return;
	};
	let retention = Retention::default();
	if store.prune(now_ms(), retention).is_err() {
		set_status(&status, StorageStatus::Unavailable);
		return;
	}
	set_status(&status, StorageStatus::Available);

	while let Ok(command) = receiver.recv() {
		let result = match command {
			StorageCommand::Append { recorded_at_ms, samples } => store
				.append_process_samples(&samples)
				.and_then(|()| store.prune(recorded_at_ms, retention)),
			StorageCommand::History { key, from_ms, to_ms, reply } => {
				let result = store.process_history(key, from_ms, to_ms);
				let succeeded = result.is_ok();
				let _ = reply.send(result);
				if succeeded {
					set_status(&status, StorageStatus::Available);
				}
				continue;
			}
			#[cfg(feature = "e2e")]
			StorageCommand::DisableForE2e => {
				set_status(&status, StorageStatus::Unavailable);
				break;
			}
		};

		if result.is_err() {
			set_status(&status, StorageStatus::Unavailable);
			break;
		}
		set_status(&status, StorageStatus::Available);
	}

	set_status(&status, StorageStatus::Unavailable);
}

fn now_ms() -> u64 {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap_or_default()
		.as_millis()
		.try_into()
		.unwrap_or(u64::MAX)
}

fn set_status(status: &RwLock<StorageStatus>, value: StorageStatus) {
	if let Ok(mut status) = status.write() {
		*status = value;
	}
}
