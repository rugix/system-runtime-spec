//! Evaluate health graphs using caller-supplied boot-time evidence.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::Error;
use crate::threshold::ValueEvidence;
use crate::types::health::*;
use crate::types::value::{ValueDefinition, ValueObservation};

/// Read the Linux boot identifier and CLOCK_BOOTTIME in milliseconds.
#[cfg(target_os = "linux")]
pub fn current_context() -> Result<HealthContext, Error> {
    let path = std::path::PathBuf::from("/proc/sys/kernel/random/boot_id");
    let boot_id = std::fs::read_to_string(&path).map_err(|source| Error::Io { path, source })?;
    let time = rustix::time::clock_gettime(rustix::time::ClockId::Boottime);
    Ok(HealthContext {
        boot_id: boot_id.trim().into(),
        now_ms: (time.tv_sec as u64) * 1_000 + (time.tv_nsec as u64) / 1_000_000,
    })
}

/// Validate the graph, then evaluate every node in declaration order.
///
/// Missing, expired, future, wrong-boot, or wrong-tag evidence yields unknown.
#[tracing::instrument(level = "debug", skip_all, fields(nodes = nodes.len(), statuses = statuses.len()))]
pub fn evaluate_health(
    nodes: &[HealthNode],
    statuses: &[HealthStatus],
    context: &HealthContext,
) -> Result<HealthReport, Error> {
    evaluate_health_with_values(nodes, statuses, &[], &[], context)
}

/// Validate numeric evidence and evaluate observed, threshold, and aggregate nodes.
#[tracing::instrument(level = "debug", skip_all)]
pub fn evaluate_health_with_values(
    nodes: &[HealthNode],
    statuses: &[HealthStatus],
    values: &[ValueDefinition],
    value_observations: &[ValueObservation],
    context: &HealthContext,
) -> Result<HealthReport, Error> {
    let evidence = ValueEvidence::new(values, value_observations)?;
    let indices: BTreeMap<_, _> = nodes
        .iter()
        .enumerate()
        .map(|(i, node)| (&node.id, i))
        .collect();
    if indices.len() != nodes.len() {
        return Err(Error::Invalid("duplicate health node ID".into()));
    }
    let observations: BTreeMap<_, _> = statuses.iter().map(|a| (&a.id, a)).collect();
    if observations.len() != statuses.len() {
        return Err(Error::Invalid("duplicate health status ID".into()));
    }
    for status in statuses {
        if status.max_age_ms == 0 {
            return Err(Error::Invalid(format!(
                "status {:?} has zero maxAgeMs",
                status.id
            )));
        }
        if indices
            .get(&status.id)
            .is_some_and(|i| !matches!(nodes[*i].rule, HealthRule::Observed(_)))
        {
            return Err(Error::Invalid(format!(
                "status supplied for derived node {:?}",
                status.id
            )));
        }
    }
    let mut remaining = vec![0usize; nodes.len()];
    let mut dependents = vec![Vec::new(); nodes.len()];
    for (index, node) in nodes.iter().enumerate() {
        match &node.rule {
            HealthRule::Observed(rule) => {
                if rule.max_age_ms == Some(0) {
                    return Err(Error::Invalid(format!(
                        "node {:?} has zero maxAgeMs",
                        node.id
                    )));
                }
            }
            HealthRule::Threshold(rule) => evidence.validate(rule)?,
            HealthRule::Aggregate(rule) => {
                if !(1 <= rule.required
                    && rule.required <= rule.healthy
                    && (rule.healthy as usize) <= rule.inputs.len())
                {
                    return Err(Error::Invalid(format!(
                        "invalid thresholds for {:?}",
                        node.id
                    )));
                }
                let unique: BTreeSet<_> = rule.inputs.iter().collect();
                if unique.len() != rule.inputs.len() {
                    return Err(Error::Invalid(format!("duplicate input for {:?}", node.id)));
                }
                remaining[index] = rule.inputs.len();
                for id in &rule.inputs {
                    let input = indices
                        .get(id)
                        .ok_or_else(|| Error::Invalid(format!("missing health node {id:?}")))?;
                    dependents[*input].push(index);
                }
            }
        }
    }
    let mut ready: VecDeque<_> = remaining
        .iter()
        .enumerate()
        .filter_map(|(i, count)| (*count == 0).then_some(i))
        .collect();
    let mut results: Vec<Option<HealthNodeResult>> = vec![None; nodes.len()];
    while let Some(index) = ready.pop_front() {
        let node = &nodes[index];
        results[index] = Some(match &node.rule {
            HealthRule::Observed(rule) => {
                observe(node, rule, observations.get(&node.id).copied(), context)
            }
            HealthRule::Threshold(rule) => evidence.evaluate(node, rule, context),
            HealthRule::Aggregate(rule) => {
                let inputs: Vec<_> = rule
                    .inputs
                    .iter()
                    .map(|id| {
                        results[indices[id]]
                            .as_ref()
                            .expect("topological inputs have been evaluated")
                    })
                    .collect();
                aggregate(node, rule, &inputs)
            }
        });
        for &dependent in &dependents[index] {
            remaining[dependent] -= 1;
            if remaining[dependent] == 0 {
                ready.push_back(dependent);
            }
        }
    }
    let nodes = results
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| Error::Invalid("cycle in health graph".into()))?;
    Ok(HealthReport { nodes })
}

/// Apply identity and finite-age constraints to one status.
fn observe(
    node: &HealthNode,
    rule: &ObservationRule,
    status: Option<&HealthStatus>,
    context: &HealthContext,
) -> HealthNodeResult {
    let reason = match status {
        None => Some("Missing status"),
        Some(a) if a.boot_id != context.boot_id => Some("Status belongs to another boot"),
        Some(a)
            if node
                .origin
                .as_ref()
                .is_some_and(|origin| a.origin.as_ref() != Some(origin)) =>
        {
            Some("Status origin does not match")
        }
        Some(a)
            if rule
                .expected_tag
                .as_ref()
                .is_some_and(|expected| a.tag.as_ref() != Some(expected)) =>
        {
            Some("Status tag does not match")
        }
        Some(a) if a.observed_at_ms > context.now_ms => Some("Status is in the future"),
        Some(a)
            if context.now_ms - a.observed_at_ms
                >= u64::from(a.max_age_ms.min(rule.max_age_ms.unwrap_or(a.max_age_ms))) =>
        {
            Some("Status has expired")
        }
        Some(_) => None,
    };
    if let Some(reason) = reason {
        return HealthNodeResult {
            id: node.id.clone(),
            origin: node.origin.clone(),
            state: HealthState::Unknown,
            inputs: Vec::new(),
            message: Some(reason.into()),
        };
    }
    let status = status.expect("absence produces a freshness reason");
    HealthNodeResult {
        id: node.id.clone(),
        origin: node.origin.clone(),
        state: status.state,
        inputs: Vec::new(),
        message: status.message.clone(),
    }
}

/// Evaluate desired and minimum counts from input health states.
fn aggregate(
    node: &HealthNode,
    rule: &AggregationRule,
    inputs: &[&HealthNodeResult],
) -> HealthNodeResult {
    let healthy = inputs
        .iter()
        .filter(|i| i.state == HealthState::Healthy)
        .count();
    let minimum_met = inputs
        .iter()
        .filter(|i| matches!(i.state, HealthState::Healthy | HealthState::Degraded))
        .count();
    let possible_minimum = inputs
        .iter()
        .filter(|i| i.state != HealthState::Unhealthy)
        .count();
    let state = if healthy >= rule.healthy as usize {
        HealthState::Healthy
    } else if minimum_met >= rule.required as usize {
        HealthState::Degraded
    } else if possible_minimum < rule.required as usize {
        HealthState::Unhealthy
    } else {
        HealthState::Unknown
    };
    HealthNodeResult {
        id: node.id.clone(),
        origin: node.origin.clone(),
        state,
        inputs: rule.inputs.clone(),
        message: None,
    }
}
