use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{self, Duration};

use rand::Rng;

/// A backoff strategy that provides durations for retrying operations with exponential backoff.
#[derive(Debug)]
pub struct Backoff {
    attempt: AtomicU64,
    factor: f64,
    jitter: bool,
    min: time::Duration,
    max: time::Duration,
}

impl Backoff {
    /// Creates a new instance of `Backoff` with default configuration.
    pub fn new() -> Self {
        Self {
            attempt: AtomicU64::new(0),
            factor: 2.0,
            jitter: false,
            min: time::Duration::from_millis(100),
            max: time::Duration::from_secs(10),
        }
    }
}

/// Returns a new instance of `Backoff` with default configuration.
impl Clone for Backoff {
    fn clone(&self) -> Self {
        Self {
            attempt: AtomicU64::new(self.attempt.load(Ordering::SeqCst)),
            factor: self.factor,
            jitter: self.jitter,
            min: self.min,
            max: self.max,
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
    pub fn next_duration(&self) -> Duration {
        let attempt = self.attempt.fetch_add(1, Ordering::SeqCst);
        self.duration_for_attempt(attempt)
    }

    /// Returns the current attempt count.
    pub fn current_attempt(&self) -> u64 {
        self.attempt.load(Ordering::SeqCst)
    }

    /// Resets the attempt count to 0.
    pub fn reset(&self) {
        self.attempt.store(0, Ordering::SeqCst);
    }

    /// Returns the backoff duration for the given attempt.
    pub fn duration_for_attempt(&self, attempt: u64) -> time::Duration {
        let base = self.min.as_secs_f64();
        let mut dur = base * self.factor.powi(attempt as i32);

        if self.jitter {
            let mut rng = rand::thread_rng();
            dur = rng.gen_range(base..=dur);
        }

        Duration::from_secs_f64(dur).clamp(self.min, self.max)
    }
}
