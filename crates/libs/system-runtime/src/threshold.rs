//! Validate numeric evidence and derive health from declared ranges.

use std::collections::BTreeMap;

use crate::Error;
use crate::types::health::{
    HealthContext, HealthNode, HealthNodeResult, HealthState, ThresholdRule,
};
use crate::types::metric::{MetricDefinition, MetricId, MetricRange, MetricSample};

/// Validated definitions and measurements shared by threshold nodes.
pub(crate) struct MetricEvidence<'a> {
    definitions: BTreeMap<&'a MetricId, &'a MetricDefinition>,
    observations: BTreeMap<&'a MetricId, &'a MetricSample>,
}

impl<'a> MetricEvidence<'a> {
    /// Validate all contributions, including measurements without a current definition.
    pub(crate) fn new(
        definitions: &'a [MetricDefinition],
        observations: &'a [MetricSample],
    ) -> Result<Self, Error> {
        let by_id: BTreeMap<_, _> = definitions.iter().map(|v| (&v.id, v)).collect();
        let samples: BTreeMap<_, _> = observations.iter().map(|v| (&v.id, v)).collect();
        if by_id.len() != definitions.len() {
            return Err(Error::Invalid("duplicate metric ID".into()));
        }
        if samples.len() != observations.len() {
            return Err(Error::Invalid("duplicate metric sample ID".into()));
        }
        for definition in definitions {
            if definition.max_age_ms == Some(0) {
                return Err(Error::Invalid("metric has zero maxAgeMs".into()));
            }
        }
        for observation in observations {
            validate_number(&observation.value)?;
            if observation.max_age_ms == 0 {
                return Err(Error::Invalid("metric sample has zero maxAgeMs".into()));
            }
        }
        Ok(Self {
            definitions: by_id,
            observations: samples,
        })
    }

    /// Ensure a threshold references a value and has nested, nonempty ranges.
    pub(crate) fn validate(&self, rule: &ThresholdRule) -> Result<(), Error> {
        if !self.definitions.contains_key(&rule.metric) {
            return Err(Error::Invalid(format!("missing metric {:?}", rule.metric)));
        }
        if rule.max_age_ms == Some(0) {
            return Err(Error::Invalid("threshold has zero maxAgeMs".into()));
        }
        validate_range(&rule.healthy)?;
        validate_range(&rule.usable)?;
        for (inner, outer, lower) in [
            (&rule.healthy.lower, &rule.usable.lower, true),
            (&rule.healthy.upper, &rule.usable.upper, false),
        ] {
            if let Some(outer) = outer {
                let Some(inner) = inner else {
                    return Err(Error::Invalid(
                        "healthy range is outside usable range".into(),
                    ));
                };
                if (lower && inner.value < outer.value)
                    || (!lower && inner.value > outer.value)
                    || (inner.value == outer.value && inner.inclusive && !outer.inclusive)
                {
                    return Err(Error::Invalid(
                        "healthy range is outside usable range".into(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Apply the referenced value's identity and freshness constraints before comparing.
    pub(crate) fn evaluate(
        &self,
        node: &HealthNode,
        rule: &ThresholdRule,
        context: &HealthContext,
    ) -> HealthNodeResult {
        let definition = self.definitions[&rule.metric];
        let observation = self.observations.get(&rule.metric).copied();
        let reason = match observation {
            None => Some("Missing metric sample"),
            Some(v) if v.boot_id != context.boot_id => {
                Some("Metric sample belongs to another boot")
            }
            Some(v)
                if definition
                    .owner
                    .as_ref()
                    .is_some_and(|o| v.owner.as_ref() != Some(o)) =>
            {
                Some("Metric sample owner does not match")
            }
            Some(v)
                if definition
                    .expected_tag
                    .as_ref()
                    .is_some_and(|t| v.tag.as_ref() != Some(t)) =>
            {
                Some("Metric sample tag does not match")
            }
            Some(v) if v.observed_at_ms > context.now_ms => Some("Metric sample is in the future"),
            Some(v)
                if context.now_ms - v.observed_at_ms
                    >= u64::from(
                        v.max_age_ms
                            .min(definition.max_age_ms.unwrap_or(v.max_age_ms))
                            .min(rule.max_age_ms.unwrap_or(v.max_age_ms)),
                    ) =>
            {
                Some("Metric sample has expired")
            }
            Some(_) => None,
        };
        if let Some(reason) = reason {
            return HealthNodeResult {
                id: node.id.clone(),
                owner: node.owner.clone(),
                state: HealthState::Unknown,
                inputs: Vec::new(),
                message: Some(reason.into()),
            };
        }
        let observation = observation.expect("absence produces a freshness reason");
        let state = if contains(&rule.healthy, &observation.value) {
            HealthState::Healthy
        } else if contains(&rule.usable, &observation.value) {
            HealthState::Degraded
        } else {
            HealthState::Unhealthy
        };
        HealthNodeResult {
            id: node.id.clone(),
            owner: node.owner.clone(),
            state,
            inputs: Vec::new(),
            message: observation.message.clone(),
        }
    }
}

/// Reject non-finite measurements and endpoints before any comparisons.
fn validate_number(value: &f64) -> Result<(), Error> {
    if !value.is_finite() {
        return Err(Error::Invalid("numeric value must be finite".into()));
    }
    Ok(())
}

/// Check endpoint finiteness and reject reversed or empty intervals.
fn validate_range(range: &MetricRange) -> Result<(), Error> {
    for bound in [&range.lower, &range.upper].into_iter().flatten() {
        validate_number(&bound.value)?;
    }
    if let (Some(lower), Some(upper)) = (&range.lower, &range.upper)
        && (lower.value > upper.value
            || (lower.value == upper.value && !(lower.inclusive && upper.inclusive)))
    {
        return Err(Error::Invalid("empty or reversed value range".into()));
    }
    Ok(())
}

/// Test both endpoints with their declared inclusion rules.
fn contains(range: &MetricRange, value: &f64) -> bool {
    range
        .lower
        .as_ref()
        .is_none_or(|b| *value > b.value || (*value == b.value && b.inclusive))
        && range
            .upper
            .as_ref()
            .is_none_or(|b| *value < b.value || (*value == b.value && b.inclusive))
}
