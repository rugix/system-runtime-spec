//! Correlate health observation times with a system's wall clock for display.

use sidex::types::Timestamp;

use crate::health::{HealthClockSample, HealthStatus};

// Use Python datetime's representable years for consistent SDK display.
const MIN_TIMESTAMP_MS: i128 = -62_135_596_800_000;
const MAX_TIMESTAMP_MS: i128 = 253_402_300_799_999;

/// Sample UTC wall time between two Linux boot-time reads.
///
/// The midpoint approximates the instant of the wall-clock sample.
#[cfg(target_os = "linux")]
pub fn current_clock_sample() -> Result<HealthClockSample, crate::Error> {
    let mut context = crate::current_context()?;
    let timestamp = Timestamp::now();
    let after = crate::current_context()?.now_ms;
    context.now_ms += (after - context.now_ms) / 2;
    Ok(HealthClockSample { context, timestamp })
}

/// Estimate a status's wall time at millisecond precision.
///
/// Returns None for another boot, a future observation, or an unrepresentable time.
/// This conversion is for display; freshness is evaluated on the boot-time clock.
pub fn status_timestamp(status: &HealthStatus, clock: &HealthClockSample) -> Option<Timestamp> {
    if status.boot_id != clock.context.boot_id {
        return None;
    }
    let age = clock.context.now_ms.checked_sub(status.observed_at_ms)?;
    let millis = i128::from(clock.timestamp.unix_seconds()) * 1_000
        + i128::from(clock.timestamp.subsec_nanos() / 1_000_000)
        - i128::from(age);
    if !(MIN_TIMESTAMP_MS..=MAX_TIMESTAMP_MS).contains(&millis) {
        return None;
    }
    let seconds =
        i64::try_from(millis.div_euclid(1_000)).expect("supported timestamps fit into i64 seconds");
    let nanos = u32::try_from(millis.rem_euclid(1_000))
        .expect("a millisecond remainder fits into u32")
        * 1_000_000;
    Timestamp::from_unix(seconds, nanos)
}
