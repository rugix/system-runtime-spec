"""Correlate the boot-time clock with wall-clock time."""

from datetime import UTC, datetime, timedelta

from ._records import Json, to_json
from .health import current_context
from .models.health import HealthClockSample


def current_clock_sample() -> HealthClockSample:
    """Sample the wall clock between two reads of the boot-time clock."""
    context = current_context()
    timestamp = datetime.now(UTC)
    after = current_context().now_ms
    context.now_ms += (after - context.now_ms) // 2
    return HealthClockSample(context=context, timestamp=timestamp)


def estimate_timestamp(
    evidence: Json, clock: HealthClockSample | Json
) -> datetime | None:
    """Estimate the wall-clock time of evidence, with millisecond precision.

    Return None for evidence from another boot, evidence observed after the clock
    sample, and estimates outside the years 0001 to 9999.
    """
    evidence = to_json(evidence)
    sample = (
        clock
        if isinstance(clock, HealthClockSample)
        else HealthClockSample.model_validate(clock)
    )
    observed = int(evidence["observedAtMs"])
    if evidence["bootId"] != sample.context.boot_id or observed > sample.context.now_ms:
        return None
    try:
        timestamp = sample.timestamp.astimezone(UTC)
        timestamp = timestamp.replace(microsecond=timestamp.microsecond // 1000 * 1000)
        return timestamp - timedelta(milliseconds=sample.context.now_ms - observed)
    except OverflowError:
        return None
