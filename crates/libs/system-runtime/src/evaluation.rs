use std::collections::{BTreeMap, BTreeSet};

use crate::Records;
use crate::records::Record;
use crate::types::common::{Diagnostic, DiagnosticKind, Severity};
use crate::types::health::*;
use crate::types::metric::{MetricDefinition, MetricRange, MetricSample};

impl HealthContext {
    /// Read the current Linux boot ID and boot-time clock.
    #[cfg(target_os = "linux")]
    pub fn current() -> std::io::Result<Self> {
        const BOOT_ID: &str = "/proc/sys/kernel/random/boot_id";
        let boot_id = std::fs::read_to_string(BOOT_ID).map_err(|error| {
            std::io::Error::new(error.kind(), format!("failed to read {BOOT_ID}: {error}"))
        })?;
        let time = rustix::time::clock_gettime(rustix::time::ClockId::Boottime);
        Ok(Self {
            boot_id: boot_id.trim().into(),
            now_ms: (time.tv_sec as u64) * 1_000 + (time.tv_nsec as u64) / 1_000_000,
        })
    }
}

impl Records {
    /// Evaluate the health records in a health context.
    ///
    /// The report contains one result per node ID, in the order of the node definitions, and
    /// the diagnostics about health records and directory configurations, including those
    /// that evaluation adds.
    #[tracing::instrument(level = "debug", skip_all)]
    pub fn evaluate_health(&self, context: &HealthContext) -> HealthReport {
        Evaluation::new(self, context).report()
    }
}

type Answers = (Answer, Answer);

const UNKNOWN: Answers = (Answer::Unknown, Answer::Unknown);

fn conjunction(answers: impl IntoIterator<Item = Answer>) -> Answer {
    let mut result = Answer::Yes;
    for answer in answers {
        match answer {
            Answer::No => return Answer::No,
            Answer::Unknown => result = Answer::Unknown,
            Answer::Yes => {}
        }
    }
    result
}

fn count_answer(minimum: u64, answers: &[Answer]) -> Answer {
    let yes = answers.iter().filter(|a| **a == Answer::Yes).count() as u64;
    let unknown = answers.iter().filter(|a| **a == Answer::Unknown).count() as u64;
    if yes >= minimum {
        Answer::Yes
    } else if yes + unknown < minimum {
        Answer::No
    } else {
        Answer::Unknown
    }
}

fn contains(range: &MetricRange, value: f64) -> bool {
    range
        .lower
        .as_ref()
        .is_none_or(|b| value > b.value || (value == b.value && b.inclusive))
        && range
            .upper
            .as_ref()
            .is_none_or(|b| value < b.value || (value == b.value && b.inclusive))
}

fn group<T: Clone>(
    items: &[Record<T>],
    id: impl Fn(&T) -> &str,
) -> BTreeMap<String, Vec<Record<T>>> {
    let mut groups: BTreeMap<String, Vec<Record<T>>> = BTreeMap::new();
    for item in items {
        groups
            .entry(id(&item.value).to_owned())
            .or_default()
            .push(item.clone());
    }
    groups
}

struct Evidence<'a> {
    boot_id: &'a str,
    observed_at_ms: u64,
    max_age_ms: u64,
    owner: Option<&'a str>,
    tag: Option<&'a str>,
}

struct Evaluation<'a> {
    context: &'a HealthContext,
    diagnostics: Vec<Diagnostic>,
    reported: BTreeSet<String>,
    order: Vec<String>,
    nodes: BTreeMap<String, Record<HealthNode>>,
    conflicts: BTreeSet<String>,
    statuses: BTreeMap<String, Vec<Record<HealthStatus>>>,
    metrics: BTreeMap<String, Vec<Record<MetricDefinition>>>,
    samples: BTreeMap<String, Vec<Record<MetricSample>>>,
    inputs: BTreeMap<String, Vec<(String, InputMode)>>,
    unsupported: BTreeSet<String>,
    on_cycle: BTreeSet<String>,
    results: BTreeMap<String, HealthNodeResult>,
}

impl<'a> Evaluation<'a> {
    fn new(records: &Records, context: &'a HealthContext) -> Self {
        let diagnostics = records.diagnostics_about(&[
            DiagnosticKind::HealthNode,
            DiagnosticKind::HealthStatus,
            DiagnosticKind::MetricDefinition,
            DiagnosticKind::MetricSample,
            DiagnosticKind::DirectoryConfig,
        ]);
        let nodes = &records.health_nodes;
        let mut order = Vec::new();
        for node in nodes {
            if !order.contains(&node.value.id.0) {
                order.push(node.value.id.0.clone());
            }
        }
        let groups = group(nodes, |node| &node.id.0);
        let mut evaluation = Self {
            context,
            diagnostics,
            reported: BTreeSet::new(),
            order,
            nodes: BTreeMap::new(),
            conflicts: BTreeSet::new(),
            statuses: group(&records.health_statuses, |status| &status.id.0),
            metrics: group(&records.metrics, |metric| &metric.id.0),
            samples: group(&records.metric_samples, |sample| &sample.id.0),
            inputs: BTreeMap::new(),
            unsupported: BTreeSet::new(),
            on_cycle: BTreeSet::new(),
            results: BTreeMap::new(),
        };
        for (id, mut group) in groups {
            if group.len() > 1 {
                evaluation.diagnose(
                    Severity::Error,
                    DiagnosticKind::HealthNode,
                    format!("several nodes have the ID {id:?}"),
                );
                evaluation.conflicts.insert(id);
            } else {
                let node = group.pop().expect("groups are not empty");
                if node.unsupported {
                    evaluation.unsupported.insert(id.clone());
                }
                evaluation.nodes.insert(id, node);
            }
        }
        evaluation.resolve_inputs();
        evaluation.on_cycle = evaluation.find_cycles();
        for id in evaluation.order.clone() {
            if evaluation.on_cycle.contains(&id) {
                evaluation.diagnose(
                    Severity::Warning,
                    DiagnosticKind::HealthNode,
                    format!("node {id:?} is on a cycle"),
                );
            }
        }
        evaluation
    }

    fn diagnose(&mut self, severity: Severity, kind: DiagnosticKind, message: String) {
        if self.reported.insert(message.clone()) {
            self.diagnostics
                .push(Diagnostic::new(severity, kind, message));
        }
    }

    fn aggregate_rule(&self, id: &str) -> Option<&AggregationRule> {
        match &self.nodes.get(id)?.value.rule {
            Some(HealthRule::Aggregate(rule)) => Some(rule),
            _ => None,
        }
    }

    fn resolve_inputs(&mut self) {
        let mut attached: BTreeMap<String, Vec<(String, InputMode)>> = BTreeMap::new();
        let mut unsupported = Vec::new();
        for id in &self.order {
            let Some(node) = self.nodes.get(id) else {
                continue;
            };
            for attachment in node.value.part_of.iter().flatten() {
                let parent = &attachment.node.0;
                if self.aggregate_rule(parent).is_none() {
                    continue;
                }
                if node.unsupported_parents.contains(parent) {
                    unsupported.push(parent.clone());
                }
                attached
                    .entry(parent.clone())
                    .or_default()
                    .push((id.clone(), attachment.mode.unwrap_or(InputMode::Counted)));
            }
        }
        self.unsupported.extend(unsupported);
        let mut missing = Vec::new();
        for id in &self.order {
            let Some(rule) = self.aggregate_rule(id) else {
                continue;
            };
            let mut inputs: Vec<(String, InputMode)> = Vec::new();
            for entry in rule.inputs.iter().flatten() {
                let child = &entry.node.0;
                if !self.nodes.contains_key(child) && !self.conflicts.contains(child) {
                    missing.push(format!("input {child:?} of {id:?} does not exist"));
                }
                if !inputs.iter().any(|(existing, _)| existing == child) {
                    inputs.push((child.clone(), entry.mode.unwrap_or(InputMode::Counted)));
                }
            }
            for (child, mode) in attached.remove(id).unwrap_or_default() {
                if !inputs.iter().any(|(existing, _)| *existing == child) {
                    inputs.push((child, mode));
                }
            }
            self.inputs.insert(id.clone(), inputs);
        }
        for message in missing {
            self.diagnose(Severity::Warning, DiagnosticKind::HealthNode, message);
        }
    }

    /// Find the nodes on a cycle with Tarjan's algorithm.
    fn find_cycles(&self) -> BTreeSet<String> {
        let edges = |id: &str| -> Vec<&str> {
            self.inputs
                .get(id)
                .into_iter()
                .flatten()
                .filter(|(child, _)| self.nodes.contains_key(child))
                .map(|(child, _)| child.as_str())
                .collect()
        };
        let mut index: BTreeMap<&str, usize> = BTreeMap::new();
        let mut low: BTreeMap<&str, usize> = BTreeMap::new();
        let mut stack: Vec<&str> = Vec::new();
        let mut on_stack: BTreeSet<&str> = BTreeSet::new();
        let mut on_cycle = BTreeSet::new();
        for start in self.inputs.keys() {
            if index.contains_key(start.as_str()) {
                continue;
            }
            let mut work = vec![(start.as_str(), 0usize)];
            let next = index.len();
            index.insert(start, next);
            low.insert(start, next);
            stack.push(start);
            on_stack.insert(start);
            while let Some(&(id, position)) = work.last() {
                let children = edges(id);
                if position < children.len() {
                    work.last_mut().expect("not empty").1 += 1;
                    let child = children[position];
                    if !index.contains_key(child) {
                        let next = index.len();
                        index.insert(child, next);
                        low.insert(child, next);
                        stack.push(child);
                        on_stack.insert(child);
                        work.push((child, 0));
                    } else if on_stack.contains(child) {
                        let value = low[id].min(index[child]);
                        low.insert(id, value);
                    }
                    continue;
                }
                work.pop();
                if let Some(&(parent, _)) = work.last() {
                    let value = low[parent].min(low[id]);
                    low.insert(parent, value);
                }
                if low[id] == index[id] {
                    let mut component = Vec::new();
                    loop {
                        let member = stack.pop().expect("member on stack");
                        on_stack.remove(member);
                        component.push(member);
                        if member == id {
                            break;
                        }
                    }
                    if component.len() > 1 || children.contains(&id) {
                        on_cycle.extend(component.into_iter().map(str::to_owned));
                    }
                }
            }
        }
        on_cycle
    }

    fn report(mut self) -> HealthReport {
        let mut nodes = Vec::new();
        for id in self.order.clone() {
            if self.conflicts.contains(&id) {
                nodes.push(HealthNodeResult {
                    id: HealthNodeId(id),
                    owner: None,
                    usable: Answer::Unknown,
                    healthy: Answer::Unknown,
                    inputs: Vec::new(),
                    reason: Some(Reason::Conflict),
                    message: None,
                });
            } else {
                nodes.push(self.result(&id));
            }
        }
        HealthReport {
            nodes,
            diagnostics: self.diagnostics,
        }
    }

    fn answers(&mut self, id: &str) -> Answers {
        if self.nodes.contains_key(id) {
            let result = self.result(id);
            (result.usable, result.healthy)
        } else {
            UNKNOWN
        }
    }

    fn result(&mut self, id: &str) -> HealthNodeResult {
        if let Some(result) = self.results.get(id) {
            return result.clone();
        }
        let node = &self.nodes[id].value;
        let mut result = HealthNodeResult {
            id: node.id.clone(),
            owner: node.owner.clone(),
            usable: Answer::Unknown,
            healthy: Answer::Unknown,
            inputs: self
                .inputs
                .get(id)
                .into_iter()
                .flatten()
                .map(|(child, _)| HealthNodeId(child.clone()))
                .collect(),
            reason: None,
            message: None,
        };
        let rule = node.rule.clone();
        if self.on_cycle.contains(id) {
            result.reason = Some(Reason::Cycle);
        } else if self.unsupported.contains(id) {
            result.reason = Some(Reason::Unsupported);
        } else {
            match rule {
                None => self.observe(
                    &mut result,
                    id,
                    &ObservationRule {
                        max_age_ms: None,
                        expected_tag: None,
                    },
                ),
                Some(HealthRule::Observed(rule)) => self.observe(&mut result, id, &rule),
                Some(HealthRule::Threshold(rule)) => self.threshold(&mut result, id, &rule),
                Some(HealthRule::Aggregate(rule)) => self.aggregate(&mut result, id, &rule),
            }
        }
        self.results.insert(id.to_owned(), result.clone());
        result
    }

    /// Return why evidence is not fresh, or None if it is fresh.
    fn freshness_reason(
        &self,
        evidence: &Evidence<'_>,
        owner: Option<&str>,
        tag: Option<&str>,
        max_ages: &[Option<u64>],
    ) -> Option<Reason> {
        if evidence.boot_id != self.context.boot_id {
            return Some(Reason::OtherBoot);
        }
        if owner.is_some_and(|owner| evidence.owner != Some(owner)) {
            return Some(Reason::OwnerMismatch);
        }
        if tag.is_some_and(|tag| evidence.tag != Some(tag)) {
            return Some(Reason::TagMismatch);
        }
        if evidence.observed_at_ms > self.context.now_ms {
            return Some(Reason::Future);
        }
        let age = self.context.now_ms - evidence.observed_at_ms;
        if std::iter::once(evidence.max_age_ms)
            .chain(max_ages.iter().flatten().copied())
            .any(|limit| age >= limit)
        {
            return Some(Reason::Expired);
        }
        None
    }

    fn observe(&mut self, result: &mut HealthNodeResult, id: &str, rule: &ObservationRule) {
        let count = self.statuses.get(id).map_or(0, Vec::len);
        if count > 1 {
            self.diagnose(
                Severity::Warning,
                DiagnosticKind::HealthStatus,
                format!("several status records for {id:?}"),
            );
            result.reason = Some(Reason::Conflict);
            return;
        }
        let Some(status) = self.statuses.get(id).and_then(|statuses| statuses.first()) else {
            result.reason = Some(Reason::Missing);
            return;
        };
        let owner = self.nodes[id].value.owner.as_deref();
        let record = &status.value;
        let evidence = Evidence {
            boot_id: &record.boot_id,
            observed_at_ms: record.observed_at_ms,
            max_age_ms: record.max_age_ms,
            owner: record.owner.as_deref(),
            tag: record.tag.as_deref(),
        };
        if let Some(reason) = self.freshness_reason(
            &evidence,
            owner,
            rule.expected_tag.as_deref(),
            &[rule.max_age_ms],
        ) {
            result.reason = Some(reason);
            return;
        }
        if status.unsupported {
            result.reason = Some(Reason::Unsupported);
        } else {
            (result.usable, result.healthy) = (record.usable, record.healthy);
        }
        result.message = record.message.clone();
    }

    fn threshold(&mut self, result: &mut HealthNodeResult, id: &str, rule: &ThresholdRule) {
        let metric = rule.metric.0.clone();
        let definitions = self.metrics.get(&metric).map_or(0, Vec::len);
        let samples = self.samples.get(&metric).map_or(0, Vec::len);
        if definitions == 0 {
            result.reason = Some(Reason::Missing);
            return;
        }
        let definition = self.metrics[&metric][0].value.clone();
        let mut conflicts = Vec::new();
        if definitions > 1 {
            conflicts.push((
                DiagnosticKind::MetricDefinition,
                format!("several definitions of {metric:?}"),
            ));
        }
        if samples > 1 {
            conflicts.push((
                DiagnosticKind::MetricSample,
                format!("several samples of {metric:?}"),
            ));
        }
        if rule
            .unit
            .as_ref()
            .is_some_and(|unit| *unit != definition.unit)
        {
            conflicts.push((
                DiagnosticKind::HealthNode,
                format!("unit of {id:?} differs from {metric:?}"),
            ));
        }
        if !conflicts.is_empty() {
            for (kind, message) in conflicts {
                self.diagnose(Severity::Warning, kind, message);
            }
            result.reason = Some(Reason::Conflict);
            return;
        }
        let Some(sample) = self
            .samples
            .get(&metric)
            .and_then(|samples| samples.first())
        else {
            result.reason = Some(Reason::Missing);
            return;
        };
        let record = &sample.value;
        let evidence = Evidence {
            boot_id: &record.boot_id,
            observed_at_ms: record.observed_at_ms,
            max_age_ms: record.max_age_ms,
            owner: record.owner.as_deref(),
            tag: record.tag.as_deref(),
        };
        if let Some(reason) = self.freshness_reason(
            &evidence,
            definition.owner.as_deref(),
            definition.expected_tag.as_deref(),
            &[definition.max_age_ms, rule.max_age_ms],
        ) {
            result.reason = Some(reason);
            return;
        }
        (result.usable, result.healthy) = if contains(&rule.healthy, record.value) {
            (Answer::Yes, Answer::Yes)
        } else if contains(&rule.usable, record.value) {
            (Answer::Yes, Answer::No)
        } else {
            (Answer::No, Answer::No)
        };
        result.message = record.message.clone();
    }

    fn aggregate(&mut self, result: &mut HealthNodeResult, id: &str, rule: &AggregationRule) {
        let inputs = self.inputs[id].clone();
        let answers: Vec<(InputMode, Answers)> = inputs
            .iter()
            .map(|(child, mode)| (*mode, self.answers(child)))
            .collect();
        let counted: Vec<Answers> = answers
            .iter()
            .filter(|(mode, _)| *mode != InputMode::Optional)
            .map(|(_, answers)| *answers)
            .collect();
        let with_mode = |wanted: InputMode| {
            answers
                .iter()
                .filter(move |(mode, _)| *mode == wanted)
                .map(|(_, answers)| *answers)
        };
        let min_healthy = rule.min_healthy.unwrap_or(counted.len() as u64);
        let min_usable = rule.min_usable.unwrap_or(min_healthy);
        let usable_answers: Vec<_> = counted.iter().map(|(usable, _)| *usable).collect();
        let healthy_answers: Vec<_> = counted.iter().map(|(_, healthy)| *healthy).collect();
        let usable = conjunction(
            std::iter::once(count_answer(min_usable, &usable_answers))
                .chain(with_mode(InputMode::Mandatory).map(|(usable, _)| usable)),
        );
        let healthy = conjunction(
            [usable, count_answer(min_healthy, &healthy_answers)]
                .into_iter()
                .chain(with_mode(InputMode::Mandatory).map(|(_, healthy)| healthy))
                .chain(with_mode(InputMode::Optional).map(|(_, healthy)| healthy)),
        );
        (result.usable, result.healthy) = (usable, healthy);
    }
}
