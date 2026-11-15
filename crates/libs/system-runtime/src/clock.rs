//! Correlate the boot-time clock with wall-clock time.

use sidex::types::Timestamp;

use crate::types::health::{HealthClockSample, HealthStatus};
use crate::types::metric::MetricSample;

// Unix milliseconds of the first and the last instant of the years 0001 to 9999.
const MIN_TIMESTAMP_MS: i128 = -62_135_596_800_000;
const MAX_TIMESTAMP_MS: i128 = 253_402_300_799_999;

/// Evidence: a status record or a metric sample.
pub trait Evidence {
    /// ID of the boot during which the evidence was taken.
    fn boot_id(&self) -> &str;
    /// Boot-time clock value, in milliseconds, when the evidence was taken.
    fn observed_at_ms(&self) -> u64;
}

impl Evidence for HealthStatus {
    fn boot_id(&self) -> &str {
        &self.boot_id
    }

    fn observed_at_ms(&self) -> u64 {
        self.observed_at_ms
    }
}

impl Evidence for MetricSample {
    fn boot_id(&self) -> &str {
        &self.boot_id
    }

    fn observed_at_ms(&self) -> u64 {
        self.observed_at_ms
    }
}

impl HealthClockSample {
    /// Sample the wall clock between two reads of the boot-time clock, at their midpoint.
    #[cfg(target_os = "linux")]
    pub fn now() -> std::io::Result<Self> {
        use crate::types::health::HealthContext;

        let mut context = HealthContext::current()?;
        let timestamp = Timestamp::now();
        let after = HealthContext::current()?.now_ms;
        context.now_ms += (after - context.now_ms) / 2;
        Ok(Self { context, timestamp })
    }

    /// Estimate the wall-clock time at which evidence was observed, with millisecond precision.
    ///
    /// Return `None` for evidence from another boot, evidence observed after the clock sample,
    /// and estimates outside the years 0001 to 9999.
    pub fn estimate_timestamp(&self, evidence: &impl Evidence) -> Option<Timestamp> {
        if evidence.boot_id() != self.context.boot_id {
            return None;
        }
        let age = self.context.now_ms.checked_sub(evidence.observed_at_ms())?;
        let millis = i128::from(self.timestamp.unix_seconds()) * 1_000
            + i128::from(self.timestamp.subsec_nanos() / 1_000_000)
            - i128::from(age);
        if !(MIN_TIMESTAMP_MS..=MAX_TIMESTAMP_MS).contains(&millis) {
            return None;
        }
        // The bounds above keep the seconds within `i64`; the remainder lies within `0..1000`.
        Timestamp::from_unix(
            millis.div_euclid(1_000) as i64,
            millis.rem_euclid(1_000) as u32 * 1_000_000,
        )
    }
}
