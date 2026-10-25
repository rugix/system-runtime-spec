"""Validate numeric evidence and derive health from declared ranges."""

import math
from collections.abc import Sequence

from .models.health import (
    HealthContext,
    HealthNode,
    HealthNodeResult,
    HealthRule_Threshold,
    HealthState_Degraded,
    HealthState_Healthy,
    HealthState_Unhealthy,
    HealthState_Unknown,
)
from .models.value import (
    ValueDefinition,
    ValueObservation,
    ValueRange,
)


class ValueEvidence:
    """Validated definitions and measurements shared by threshold nodes."""

    def __init__(
        self,
        definitions: Sequence[ValueDefinition],
        observations: Sequence[ValueObservation],
    ) -> None:
        self.definitions = {v.id: v for v in definitions}
        self.observations = {v.id: v for v in observations}
        if len(self.definitions) != len(definitions):
            raise ValueError("Duplicate value ID")
        if len(self.observations) != len(observations):
            raise ValueError("Duplicate value observation ID")
        for definition in definitions:
            if definition.max_age_ms == 0:
                raise ValueError("Value has zero maxAgeMs")
        for observation in observations:
            validate_number(observation.value)
            if observation.max_age_ms == 0:
                raise ValueError("Value observation has zero maxAgeMs")

    def validate(self, rule: HealthRule_Threshold) -> None:
        """Require a value definition and nested, nonempty ranges."""
        if rule.value not in self.definitions:
            raise ValueError(f"Missing value {rule.value!r}")
        if rule.max_age_ms == 0:
            raise ValueError("Threshold has zero maxAgeMs")
        for interval in (rule.healthy, rule.usable):
            for bound in (interval.lower, interval.upper):
                if bound is not None:
                    validate_number(bound.value)
            if interval.lower is not None and interval.upper is not None:
                lower, upper = (
                    interval.lower.value,
                    interval.upper.value,
                )
                if lower > upper or (
                    lower == upper
                    and not (interval.lower.inclusive and interval.upper.inclusive)
                ):
                    raise ValueError("Empty or reversed value range")
        for inner, outer, lower_bound in (
            (rule.healthy.lower, rule.usable.lower, True),
            (rule.healthy.upper, rule.usable.upper, False),
        ):
            if outer is not None:
                if inner is None:
                    raise ValueError("Healthy range is outside usable range")
                a, b = inner.value, outer.value
                if (
                    (lower_bound and a < b)
                    or (not lower_bound and a > b)
                    or (a == b and inner.inclusive and not outer.inclusive)
                ):
                    raise ValueError("Healthy range is outside usable range")

    def evaluate(self, node: HealthNode, context: HealthContext) -> HealthNodeResult:
        """Check value identity and freshness before applying the policy."""
        rule = node.rule
        assert isinstance(rule, HealthRule_Threshold)
        definition = self.definitions[rule.value]
        sample = self.observations.get(rule.value)
        reason = None
        if sample is None:
            reason = "Missing value observation"
        elif sample.boot_id != context.boot_id:
            reason = "Value belongs to another boot"
        elif definition.origin is not None and sample.origin != definition.origin:
            reason = "Value origin does not match"
        elif (
            definition.expected_tag is not None
            and sample.tag != definition.expected_tag
        ):
            reason = "Value tag does not match"
        elif sample.observed_at_ms > context.now_ms:
            reason = "Value is in the future"
        elif context.now_ms - sample.observed_at_ms >= min(
            sample.max_age_ms,
            definition.max_age_ms
            if definition.max_age_ms is not None
            else sample.max_age_ms,
            rule.max_age_ms if rule.max_age_ms is not None else sample.max_age_ms,
        ):
            reason = "Value has expired"
        if reason is not None:
            return HealthNodeResult(
                id=node.id,
                origin=node.origin,
                state=HealthState_Unknown("unknown"),
                inputs=[],
                message=reason,
            )
        assert sample is not None
        state = (
            HealthState_Healthy("healthy")
            if contains(rule.healthy, sample.value)
            else HealthState_Degraded("degraded")
            if contains(rule.usable, sample.value)
            else HealthState_Unhealthy("unhealthy")
        )
        return HealthNodeResult(
            id=node.id,
            origin=node.origin,
            state=state,
            usable=state.root != "unhealthy",
            inputs=[],
            message=sample.message,
        )


def validate_number(value: float) -> None:
    """Reject non-finite measurements and endpoints."""
    if not math.isfinite(value):
        raise ValueError("Numeric value must be finite")


def contains(interval: ValueRange, value: float) -> bool:
    """Test both endpoints with their declared inclusion rules."""
    return (
        interval.lower is None
        or value > interval.lower.value
        or (value == interval.lower.value and interval.lower.inclusive)
    ) and (
        interval.upper is None
        or value < interval.upper.value
        or (value == interval.upper.value and interval.upper.inclusive)
    )
