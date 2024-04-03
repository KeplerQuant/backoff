use crate::backoff::*;
use std::time;

#[test]
fn backoff_duration_increases_with_attempt_count() {
    let backoff = Backoff::new();

    // First attempt.
    let duration = backoff.duration();
    assert_eq!(duration, time::Duration::from_millis(100));

    // Second attempt.
    let duration = backoff.duration();
    assert_eq!(duration, time::Duration::from_millis(200));

    // Third attempt.
    let duration = backoff.duration();
    assert_eq!(duration, time::Duration::from_millis(400));
}

#[test]
fn backoff_duration_is_capped_at_max_duration() {
    let backoff = Backoff::new().with_max(time::Duration::from_millis(400));

    // First attempt.
    let duration = backoff.duration();
    assert_eq!(duration, time::Duration::from_millis(100));

    // Second attempt.
    let duration = backoff.duration();
    assert_eq!(duration, time::Duration::from_millis(200));

    // Third attempt.
    let duration = backoff.duration();
    assert_eq!(duration, time::Duration::from_millis(400));

    // Fourth attempt.
    let duration = backoff.duration();
    assert_eq!(duration, time::Duration::from_millis(400));
}

#[test]
fn backoff_duration_can_be_jittered() {
    let backoff = Backoff::new().with_jitter(true);

    // First attempt.
    let duration = backoff.duration();
    assert!(duration >= time::Duration::from_millis(100));
    assert!(duration <= time::Duration::from_millis(200));

    // Second attempt.
    let duration = backoff.duration();
    assert!(duration >= time::Duration::from_millis(200));
    assert!(duration <= time::Duration::from_millis(400));

    // Third attempt.
    let duration = backoff.duration();
    assert!(duration >= time::Duration::from_millis(400));
    assert!(duration <= time::Duration::from_secs(1));
}

#[test]
fn backoff_reset_can_be_reseted() {
    let backoff = Backoff::new();

    let duration = backoff.duration();
    backoff.reset();

    assert!(duration == time::Duration::from_millis(100));
}
