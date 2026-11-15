"""Correlate health observation times with a system's wall clock for display."""

from datetime import UTC, datetime, timedelta

from .health import current_context
from .models.health import HealthClockSample, HealthStatus


def current_clock_sample() -> HealthClockSample:
    """Sample UTC between two Linux boot-time reads, using their midpoint."""
    context = current_context()
    timestamp = datetime.now(UTC)
    after = current_context().now_ms
    context.now_ms += (after - context.now_ms) // 2
    return HealthClockSample(context=context, timestamp=timestamp)


def status_timestamp(status: HealthStatus, clock: HealthClockSample) -> datetime | None:
    """Estimate UTC at millisecond precision, independently of freshness checks.

    Return None for another boot, a future observation, or an unrepresentable time.
    """
    if (
        status.boot_id != clock.context.boot_id
        or status.observed_at_ms > clock.context.now_ms
    ):
        return None
    try:
        timestamp = clock.timestamp.astimezone(UTC)
        timestamp = timestamp.replace(microsecond=timestamp.microsecond // 1000 * 1000)
        return timestamp - timedelta(
            milliseconds=clock.context.now_ms - status.observed_at_ms
        )
    except OverflowError:
        return None
