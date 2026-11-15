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
            .is_some_and(|i| !matches!(node_rule(&nodes[*i]), HealthRule::Observed(_)))
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
        match node_rule(node) {
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
                if rule
                    .inputs
                    .iter()
                    .any(|input| input.required == Some(true) && input.optional == Some(true))
                {
                    return Err(Error::Invalid(format!(
                        "input both required and optional for {:?}",
                        node.id
                    )));
                }
                let (required, healthy) = counts(rule);
                let counted = rule
                    .inputs
                    .iter()
                    .filter(|input| !is_optional(input))
                    .count();
                if !(1 <= required && required <= healthy && healthy <= counted) {
                    return Err(Error::Invalid(format!(
                        "invalid thresholds for {:?}",
                        node.id
                    )));
                }
                let unique: BTreeSet<_> = rule.inputs.iter().map(|input| &input.node).collect();
                if unique.len() != rule.inputs.len() {
                    return Err(Error::Invalid(format!("duplicate input for {:?}", node.id)));
                }
                remaining[index] = rule.inputs.len();
                for dependency in &rule.inputs {
                    let id = &dependency.node;
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
        results[index] = Some(match node_rule(node) {
            HealthRule::Observed(rule) => {
                observe(node, rule, observations.get(&node.id).copied(), context)
            }
            HealthRule::Threshold(rule) => evidence.evaluate(node, rule, context),
            HealthRule::Aggregate(rule) => {
                let inputs: Vec<_> = rule
                    .inputs
                    .iter()
                    .map(|input| {
                        results[indices[&input.node]]
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
                .owner
                .as_ref()
                .is_some_and(|owner| a.owner.as_ref() != Some(owner)) =>
        {
            Some("Status owner does not match")
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
            owner: node.owner.clone(),
            state: HealthState::Unknown,
            inputs: Vec::new(),
            message: Some(reason.into()),
        };
    }
    let status = status.expect("absence produces a freshness reason");
    HealthNodeResult {
        id: node.id.clone(),
        owner: node.owner.clone(),
        state: status.state,
        inputs: Vec::new(),
        message: status.message.clone(),
    }
}

/// Rule of nodes that omit their rule: observed without constraints.
const OBSERVED: HealthRule = HealthRule::Observed(ObservationRule {
    max_age_ms: None,
    expected_tag: None,
});

/// Resolve a node's rule, treating an omitted rule as observed.
fn node_rule(node: &HealthNode) -> &HealthRule {
    node.rule.as_ref().unwrap_or(&OBSERVED)
}

/// Check whether an input only affects the desired criteria.
fn is_optional(input: &AggregationInput) -> bool {
    input.optional == Some(true)
}

/// Resolve the minimum and desired counts, defaulting to all non-optional inputs healthy.
fn counts(rule: &AggregationRule) -> (usize, usize) {
    let counted = rule
        .inputs
        .iter()
        .filter(|input| !is_optional(input))
        .count();
    let healthy = rule.healthy.map_or(counted, |healthy| healthy as usize);
    let required = rule.required.map_or(healthy, |required| required as usize);
    (required, healthy)
}

/// Apply mandatory dependency gates and desired and minimum input counts.
fn aggregate(
    node: &HealthNode,
    rule: &AggregationRule,
    inputs: &[&HealthNodeResult],
) -> HealthNodeResult {
    let (required, desired) = counts(rule);
    let (optional, counted): (Vec<_>, Vec<_>) = rule
        .inputs
        .iter()
        .zip(inputs)
        .partition(|(input, _)| is_optional(input));
    let healthy = counted
        .iter()
        .filter(|(_, i)| i.state == HealthState::Healthy)
        .count();
    let minimum_met = counted
        .iter()
        .filter(|(_, i)| matches!(i.state, HealthState::Healthy | HealthState::Degraded))
        .count();
    let possible_minimum = counted
        .iter()
        .filter(|(_, i)| i.state != HealthState::Unhealthy)
        .count();
    let mandatory: Vec<_> = rule
        .inputs
        .iter()
        .zip(inputs)
        .filter(|(input, _)| input.required == Some(true))
        .map(|(_, result)| *result)
        .collect();
    let state = if possible_minimum < required
        || mandatory
            .iter()
            .any(|input| input.state == HealthState::Unhealthy)
    {
        HealthState::Unhealthy
    } else if minimum_met < required
        || mandatory
            .iter()
            .any(|input| input.state == HealthState::Unknown)
    {
        HealthState::Unknown
    } else if healthy >= desired
        && mandatory
            .iter()
            .all(|input| input.state == HealthState::Healthy)
        && optional
            .iter()
            .all(|(_, input)| input.state == HealthState::Healthy)
    {
        HealthState::Healthy
    } else {
        HealthState::Degraded
    };
    HealthNodeResult {
        id: node.id.clone(),
        owner: node.owner.clone(),
        state,
        inputs: rule.inputs.iter().map(|input| input.node.clone()).collect(),
        message: None,
    }
}
