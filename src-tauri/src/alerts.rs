use std::collections::{HashMap, HashSet};

use crate::model::{AlertEvidence, ProcessKey};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SamplingContinuity {
	Continuous,
	Gap,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Evaluation {
	Pending,
	Triggered(AlertEvidence),
	Cooldown,
	Reset,
}

#[derive(Clone, Copy, Debug)]
struct Episode {
	high_since_ms: u64,
	triggered: bool,
}

#[derive(Debug)]
pub struct CpuAlertRule {
	threshold_percent: f32,
	sustained_duration_ms: u64,
	cooldown_ms: u64,
	episodes: HashMap<ProcessKey, Episode>,
	last_triggered_at: HashMap<ProcessKey, u64>,
}

impl CpuAlertRule {
	pub fn new(
		threshold_percent: f32,
		sustained_duration_ms: u64,
		cooldown_ms: u64,
	) -> Result<Self, &'static str> {
		if !threshold_percent.is_finite() || threshold_percent <= 0.0 {
			return Err("CPU threshold must be a finite positive percentage");
		}

		Ok(Self {
			threshold_percent,
			sustained_duration_ms,
			cooldown_ms,
			episodes: HashMap::new(),
			last_triggered_at: HashMap::new(),
		})
	}

	pub fn evaluate(
		&mut self,
		key: ProcessKey,
		recorded_at_ms: u64,
		cpu_percent: Option<f32>,
		continuity: SamplingContinuity,
	) -> Evaluation {
		if continuity == SamplingContinuity::Gap {
			self.episodes.remove(&key);
			return Evaluation::Reset;
		}

		let Some(observed_cpu_percent) = cpu_percent.filter(|value| value.is_finite()) else {
			self.episodes.remove(&key);
			return Evaluation::Reset;
		};

		if observed_cpu_percent < self.threshold_percent {
			self.episodes.remove(&key);
			return Evaluation::Reset;
		}

		let episode = self.episodes.entry(key.clone()).or_insert(Episode {
			high_since_ms: recorded_at_ms,
			triggered: false,
		});

		if episode.triggered {
			return Evaluation::Cooldown;
		}

		if recorded_at_ms.saturating_sub(episode.high_since_ms) < self.sustained_duration_ms {
			return Evaluation::Pending;
		}

		if self.last_triggered_at.get(&key).is_some_and(|last_triggered| {
			recorded_at_ms.saturating_sub(*last_triggered) < self.cooldown_ms
		}) {
			return Evaluation::Cooldown;
		}

		episode.triggered = true;
		self.last_triggered_at.insert(key.clone(), recorded_at_ms);
		Evaluation::Triggered(AlertEvidence::sustained_cpu(
			key,
			"Unknown process".to_owned(),
			observed_cpu_percent,
			self.threshold_percent,
			self.sustained_duration_ms,
			episode.high_since_ms,
			recorded_at_ms,
		))
	}

	pub fn retain_active(&mut self, active_keys: &HashSet<ProcessKey>) {
		self.episodes.retain(|key, _| active_keys.contains(key));
		self.last_triggered_at.retain(|key, _| active_keys.contains(key));
	}

	pub fn reset_continuity(&mut self) {
		self.episodes.clear();
	}
}

impl Default for CpuAlertRule {
	fn default() -> Self {
		Self::new(80.0, 600_000, 300_000).expect("default CPU alert rule must be valid")
	}
}
