use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time;

use rand::Rng;

/// A backoff strategy that provides durations for retrying operations with exponential backoff.
#[derive(Debug, Clone)]
pub struct Backoff {
    attempt: Arc<AtomicU64>,
    factor: f64,
    jitter: bool,
    min: time::Duration,
    max: time::Duration,
}

impl Backoff {
    /// Creates a new instance of `Backoff` with default configuration.
    pub fn new() -> Self {
        Self {
            attempt: Arc::new(AtomicU64::new(0)),
            factor: 2.0,
            jitter: false,
            min: time::Duration::from_millis(100),
            max: time::Duration::from_secs(10),
        }
    }
}

impl Backoff {
    /// Sets the minimum duration for backoff.
    pub fn with_min(mut self, min: time::Duration) -> Self {
        self.min = min;
        self
    }

    /// Sets the maximum duration for backoff.
    pub fn with_max(mut self, max: time::Duration) -> Self {
        self.max = max;
        self
    }

    /// Sets whether to apply jitter to the backoff durations.
    pub fn with_jitter(mut self, jitter: bool) -> Self {
        self.jitter = jitter;
        self
    }

    /// Sets the factor used for exponential backoff calculations.
    pub fn with_factor(mut self, factor: f64) -> Self {
        self.factor = factor;
        self
    }

    /// Returns the next duration for backoff.
    pub fn duration(&self) -> time::Duration {
        let attempt = self.attempt.fetch_add(1, Ordering::SeqCst);
        self.for_attempt(attempt as f64)
    }

    /// Returns the current attempt count.
    pub fn attempt(&self) -> u64 {
        self.attempt.load(Ordering::SeqCst)
    }

    /// Resets the attempt count to 0.
    pub fn reset(&self) {
        self.attempt.store(0, Ordering::SeqCst);
    }

    /// Returns the backoff duration for the given attempt.
    pub fn for_attempt(&self, attempt: f64) -> time::Duration {
        let (min, max, factor) = (self.min, self.max, self.factor);

        let minf = self.min.as_secs_f64();
        let durf = minf * factor.powf(attempt);

        let durf = if self.jitter {
            let mut rng = rand::thread_rng();
            rng.gen_range(minf..=durf) + minf
        } else {
            durf
        };

        let dur = time::Duration::from_secs_f64(durf);
        if dur < min {
            return min;
        }
        if dur > max {
            return max;
        }
        dur
    }
}
