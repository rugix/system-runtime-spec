//! Correlate the boot-time clock with wall-clock time.

use sidex::types::{JsonValue, Timestamp};

use crate::types::health::HealthClockSample;

// Unix milliseconds of the first and the last instant of the years 0001 to 9999.
const MIN_TIMESTAMP_MS: i128 = -62_135_596_800_000;
const MAX_TIMESTAMP_MS: i128 = 253_402_300_799_999;

/// Sample the wall clock between two reads of the boot-time clock, at their midpoint.
#[cfg(target_os = "linux")]
pub fn current_clock_sample() -> Result<HealthClockSample, crate::Error> {
    let mut context = crate::evaluation::current_context()?;
    let timestamp = Timestamp::now();
    let after = crate::evaluation::current_context()?.now_ms;
    context.now_ms += (after - context.now_ms) / 2;
    Ok(HealthClockSample { context, timestamp })
}

/// Estimate the wall-clock time at which evidence was observed, with millisecond precision.
///
/// Return None for evidence from another boot, evidence observed after the clock sample, and
/// estimates outside the years 0001 to 9999.
pub fn estimate_timestamp(evidence: &JsonValue, clock: &HealthClockSample) -> Option<Timestamp> {
    let boot_id = evidence.get("bootId")?.as_str()?;
    let observed_at_ms = evidence.get("observedAtMs").and_then(|value| {
        value
            .as_u64()
            .or_else(|| value.as_str().and_then(|text| text.parse().ok()))
    })?;
    if boot_id != clock.context.boot_id {
        return None;
    }
    let age = clock.context.now_ms.checked_sub(observed_at_ms)?;
    let millis = i128::from(clock.timestamp.unix_seconds()) * 1_000
        + i128::from(clock.timestamp.subsec_nanos() / 1_000_000)
        - i128::from(age);
    if !(MIN_TIMESTAMP_MS..=MAX_TIMESTAMP_MS).contains(&millis) {
        return None;
    }
    let seconds = i64::try_from(millis.div_euclid(1_000)).ok()?;
    let nanos = u32::try_from(millis.rem_euclid(1_000)).ok()? * 1_000_000;
    Timestamp::from_unix(seconds, nanos)
}
