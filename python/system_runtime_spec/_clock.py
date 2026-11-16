import time
from datetime import UTC, datetime, timedelta
from pathlib import Path

from .models.health import HealthClockSample, HealthContext, HealthStatus
from .models.metric import MetricSample


def current_context() -> HealthContext:
    """Read the current Linux boot ID and boot-time clock."""
    return HealthContext(
        boot_id=Path("/proc/sys/kernel/random/boot_id").read_text().strip(),
        now_ms=time.clock_gettime_ns(time.CLOCK_BOOTTIME) // 1_000_000,
    )


def current_clock_sample() -> HealthClockSample:
    """Sample the wall clock between two reads of the boot-time clock."""
    context = current_context()
    timestamp = datetime.now(UTC)
    after = current_context().now_ms
    context.now_ms += (after - context.now_ms) // 2
    return HealthClockSample(context=context, timestamp=timestamp)


def estimate_timestamp(
    clock: HealthClockSample, evidence: HealthStatus | MetricSample
) -> datetime | None:
    """Estimate the wall-clock time at which evidence was observed, in milliseconds.

    Return None for evidence from another boot, evidence observed after the clock
    sample, and estimates outside the years 0001 to 9999.
    """
    observed = evidence.observed_at_ms
    if evidence.boot_id != clock.context.boot_id or observed > clock.context.now_ms:
        return None
    try:
        timestamp = clock.timestamp.astimezone(UTC)
        timestamp = timestamp.replace(microsecond=timestamp.microsecond // 1000 * 1000)
        return timestamp - timedelta(milliseconds=clock.context.now_ms - observed)
    except OverflowError:
        return None
