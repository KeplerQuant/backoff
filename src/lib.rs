/// A backoff algorithm that exponentially increases the time between attempts.
///
/// Backoff is a technique that can be used to implement retries with increasing
/// intervals between attempts. This can be useful for operations that are likely to
/// fail temporarily, such as network requests or database queries.
///
/// This backoff algorithm works by starting with a minimum interval and multiplying
/// that interval by a factor after each attempt. The maximum interval can be
/// specified to prevent the interval from growing too large.
///
/// The backoff algorithm also supports jitter, which adds a random amount of time
/// to the interval. This can help to reduce contention between multiple clients that
/// are retrying the same operation.
///
/// To use the backoff algorithm, simply create a new instance and call the
/// `duration()` method to get the next interval. If you need to retry an
/// operation, call the `duration()` method again and wait for the interval to
/// elapse before trying again.
pub mod backoff;

#[cfg(test)]
mod backoff_tests;
