# System Runtime Specification

Version 1.0 (draft)

The System Runtime Specification is a common format for describing a system's components and its
health at runtime. A _system_ is everything that makes up a device, such as its system image, its
apps, and its hardware modules. Each part publishes small records about itself. Tools such as
update controllers, diagnostic tools, and fleet-management agents read the records to check
whether the components are compatible with each other and to evaluate the health of the system and
its functions.

This document is the normative definition of the specification. The
[README](README.md) gives an overview and links to implementations.

## Contents

1. [Conventions](#1-conventions)
2. [Concepts](#2-concepts)
3. [Conformance](#3-conformance)
4. [Records](#4-records)
5. [Diagnostics](#5-diagnostics)
6. [Versions and Version Requirements](#6-versions-and-version-requirements)
7. [Components](#7-components)
8. [Health](#8-health)
9. [Links Between Components and Health](#9-links-between-components-and-health)
10. [Discovery](#10-discovery)
11. [Transport](#11-transport)
12. [Evolution](#12-evolution)
13. [Security Considerations](#13-security-considerations)

## 1. Conventions

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY, and
OPTIONAL in this document are to be interpreted as described in [BCP 14][bcp14] (RFC 2119 and
RFC 8174) when, and only when, they appear in all capitals.

Examples and notes are informative. Everything else is normative, except where a section states
otherwise.

This document and the [schemas](schemas/) together form the specification. The schemas define the
structure of records, reports, and snapshots in [Sidex][sidex], a language for data models, and
their JSON representation follows [Sidex's JSON mapping][sidex-json]. This document defines their
meaning and the conditions that the structure alone does not express. It quotes the schema
definitions it refers to; if a quote differs from the schemas, the schemas apply.
The [JSON Schemas](json-schema/) generated from the schemas are informative.

[bcp14]: https://www.rfc-editor.org/info/bcp14
[sidex]: https://github.com/silitics/sidex
[sidex-json]: https://oss.silitics.com/sidex/docs/guides/json/

## 2. Concepts

The parts of a system, such as its system image and its apps, are _contributors_ that publish
_records_ about themselves. There are five kinds of records, each with a record type in the
schemas:

- A **component declaration** (`ComponentDeclaration`) describes a component of the system, such as
  the system image, an app, or a runtime, with what it provides and what it requires.
- A **health node** (`HealthNode`) describes a check or a function of the system and how its health
  is determined.
- A **status record** (`HealthStatus`) holds the result of a check.
- A **metric definition** (`MetricDefinition`) declares a measurement, such as a CPU temperature.
- A **metric sample** (`MetricSample`) holds the current value of a metric.

Status records and metric samples are _evidence_: results of checks and measurements on the running
system, which are valid for a limited time.

Tools process records into three kinds of output:

- A **compatibility report** states whether the components are compatible with each other.
- A **health report** states for each health node whether it is usable and whether it is healthy.
- A **snapshot** contains the records of a system, so that another system can check or evaluate
  them.

Records that are invalid or cannot be read are skipped and reported as _diagnostics_, which
accompany the records that tools read and are part of every output.

The specification distinguishes five roles:

- A **writer** publishes records, snapshots, or directory configurations
  ([Section 10.4](#104-directory-configurations)).
- A **reader** discovers and parses records, or receives snapshots, and provides the records with
  their diagnostics.
- A **compatibility checker** checks component declarations and produces compatibility reports.
- A **health evaluator** evaluates health nodes and evidence and produces health reports.
- A **consumer** acts on reports, such as an update controller that decides whether to commit an
  update.

An implementation can take several roles. For example, an update controller reads records, checks
the compatibility of an update, writes the records of the update, and decides whether to commit it.

## 3. Conformance

The specification consists of a foundation and two parts that build on it:

- The **foundation** covers records, diagnostics, versions, discovery, transport, and evolution
  ([Sections 4](#4-records), [5](#5-diagnostics), [6](#6-versions-and-version-requirements),
  [10](#10-discovery), [11](#11-transport), and [12](#12-evolution)).
- The **components part** covers component declarations and compatibility checking
  ([Section 7](#7-components)).
- The **health part** covers health graphs, status records, metrics, and health evaluation
  ([Section 8](#8-health)).

The two parts are independent. An implementation MAY implement either part or both, together
with the foundation. [Section 9](#9-links-between-components-and-health) defines how records of
the two parts refer to each other.

The sections that matter most for each role are:

- **Writers:** [Sections 4](#4-records), [7.1–7.3](#71-component-declarations) for component
  declarations, [8.1–8.7](#81-health-answers) for health records, and [10](#10-discovery).
- **Readers:** [Sections 4](#4-records), [5](#5-diagnostics), [10](#10-discovery),
  [11](#11-transport), and [12](#12-evolution).
- **Compatibility checkers:** [Sections 4](#4-records), [5](#5-diagnostics),
  [6](#6-versions-and-version-requirements), [7](#7-components), and [12](#12-evolution).
- **Health evaluators:** [Sections 4](#4-records), [5](#5-diagnostics), [8](#8-health), and
  [12](#12-evolution).
- **Consumers:** [Sections 7.5](#75-compatibility-reports), [8.1](#81-health-answers),
  [8.8](#88-evaluation-and-reports), and [12](#12-evolution).

An implementation conforms to the specification for a role if it satisfies all requirements that
apply to that role. The [conformance suite](tests/fixtures/) tests all parts.

## 4. Records

This section applies to all kinds of records.

### 4.1 Encoding

Each record is a JSON object of one of the record types of the schemas, such as
`ComponentDeclaration` or `HealthStatus`. Field names use `camelCase`, so the schema field
`max_age_ms` is the JSON field `maxAgeMs`. Enumeration values use `PascalCase`, and absent
optional fields are omitted. Fields of the type `u64`, such as boot-time clock values, hold
unsigned 64-bit integers; values beyond 2⁵³ − 1 are decimal strings, so that JavaScript
implementations can process them without losing precision.

A record is _valid_ if it has the structure of its schema and its fields satisfy the conditions that
this document and the schemas state for them, such as that a version is valid or that a `maxAgeMs`
is positive. Unknown fields, unknown enumeration values ([Section 12](#12-evolution)), and the unit
codes of metrics ([Section 8.5](#85-metrics)) do not make a record invalid. Each record is valid or
invalid on its own; rules that involve several records, such as unique IDs, belong to compatibility
checking and health evaluation, which define what happens when they are broken. Writers MUST NOT
publish invalid records.

### 4.2 Identifiers

Component declarations, health nodes, and metric definitions are identified by IDs; status records
refer to health nodes by their IDs, and metric samples refer to metrics. IDs, owners
([Section 4.3](#43-owners)), and the IDs of capabilities and claims ([Section 7](#7-components)) are
non-empty strings of ASCII letters, digits, and ASCII punctuation characters, such as `.`, `-`, and
`_`. They contain no spaces and no other characters. They are case-sensitive, compared by exact
equality, and sorted by their bytes.

Components, health nodes, and metrics each have their own namespace: a component, a health node, and
a metric MAY share an ID without referring to each other. Within a namespace, the records of all
contributors share the IDs. The IDs of components, health nodes, and metrics SHOULD therefore start
with a prefix that the contributor controls, followed by a dot, such as `telemetry.database`.

### 4.3 Owners

Component declarations, health nodes, status records, metric definitions, and metric samples have
an optional `owner`. The owner names the contributor that supplies the record, such as a system
image or an app, and that replaces or removes the record as a whole.

Owners do not scope IDs: records of different owners need distinct IDs within their namespace.

An update of a contributor replaces all component declarations, health nodes, and metric definitions
of its owner. A writer that performs an update MUST ensure that, once the updated records take
effect, no such records of the replaced owner from before the update remain in the directories that
readers read. A writer that rolls back an update SHOULD restore the records that the update removed.

> **Example:** A system image might use the owner `rugix.system`, and an app named `telemetry` the
> owner `rugix.apps.telemetry`.

### 4.4 Annotations

Records can carry `annotations`, data for tools that this specification does not define. Member
names SHOULD start with a reverse domain name that the writer controls, such as
`com.example.priority`. Annotations do not affect compatibility checking or health evaluation.
Writers MUST put data that this specification does not define into annotations, or into the
`details` of evidence ([Section 8.3](#83-evidence)), and MUST NOT add other fields to records.

### 4.5 Specification Versions

Every record has an optional field `specVersion`, which holds the version of this specification that
the record follows, such as `1.0`. A record without it follows version 1.0. Writers SHOULD set it to
the lowest version whose features the record uses. A specification version, here and in snapshots
([Section 11.1](#111-snapshots)), consists of two decimal numbers without leading zeros, separated
by a dot. Both numbers fit into an unsigned 64-bit integer and compare numerically.

A record whose `specVersion` does not have this form, or has a major version that the reader does
not support, is invalid. [Section 12](#12-evolution) defines how readers treat records of newer
minor versions.

## 5. Diagnostics

One faulty record should not prevent the evaluation of the others. Readers, compatibility checkers,
and health evaluators MUST skip invalid records and records that cannot be read, and MUST include a
_diagnostic_ for each in their output: readers with the records they provide, compatibility
checkers in their compatibility reports, and health evaluators in their health reports. A
diagnostic has the following structure:

```sidex
/// A record that was skipped or used despite a problem.
record Diagnostic {
    /// Whether the results may not reflect the records concerned.
    severity: Severity,
    /// Kind of the records or directory configurations concerned.
    kind: DiagnosticKind,
    /// Description of the problem; not specified and differs between implementations.
    message: string,
    /// Path of the file or directory concerned.
    path?: string,
}

/// Effect of a diagnostic on the results.
#[json(tagged = externally)]
variant Severity {
    /// The results may not reflect the records concerned, for example, because they were skipped.
    Error,
    /// The results account for the problem.
    Warning,
}

/// Kind of the files that a diagnostic concerns, named after their types.
#[json(tagged = externally)]
variant DiagnosticKind {
    /// Component declarations.
    ComponentDeclaration,
    /// Health nodes.
    HealthNode,
    /// Status records.
    HealthStatus,
    /// Metric definitions.
    MetricDefinition,
    /// Metric samples.
    MetricSample,
    /// Directory configurations.
    DirectoryConfig,
}
```

A diagnostic has the kind of the records it concerns. A diagnostic about a directory has the kind of
the records in that directory, and the kind `DirectoryConfig` is only for problems with directory
configurations ([Section 10.4](#104-directory-configurations)) themselves. Compatibility checking
and health evaluation define how diagnostics affect their results ([Sections 7.4](#74-consistency)
and [8.8](#88-evaluation-and-reports)). The order of diagnostics is not specified.

## 6. Versions and Version Requirements

Components declare versions, and other components declare requirements on them. Because the parts
of a system follow different versioning schemes, such as Semantic Versioning for libraries and
Calendar Versioning for system images, versions use one grammar that covers these schemes without
knowing which one applies.

### 6.1 Versions

A version consists of one or more numeric _release components_, an optional _prerelease_, and
optional _build metadata_:

```text
version    = release [ "-" prerelease ] [ "+" build ]
release    = number { "." number }
prerelease = identifier { "." identifier }
build      = identifier { "." identifier }
number     = digit { digit }
identifier = ( letter | digit | "-" ) { letter | digit | "-" }
```

Letters are the ASCII letters `A` to `Z` and `a` to `z`, and digits are `0` to `9`. Release
components and prerelease identifiers that consist only of digits are _numeric_ and MUST fit into an
unsigned 64-bit integer. Leading zeros are allowed. Whitespace at the start and end of a version is
ignored. In this document, _whitespace_ means the characters with the Unicode property
`White_Space`.

> **Example:** `1.4.2`, `2.0.0-rc.1`, `1.4.2+build.7`, `2026.06.25`, and `20260625071935` are
> versions. Versions of other formats need to be converted, such as the Debian version `2.27~rc1`
> to `2.27-rc.1`.

### 6.2 Precedence

Versions are ordered by _precedence_:

1. Release components compare numerically from left to right. A missing component counts as zero,
   so `5`, `5.0`, and `5.0.0` have the same precedence.
2. With equal release components, a version with a prerelease precedes the version without one, so
   `1.0-rc.1` precedes `1.0`.
3. Two prereleases compare identifier by identifier. Numeric identifiers compare numerically and
   precede other identifiers, which compare lexicographically by ASCII code. If all identifiers of
   one prerelease equal the leading identifiers of the other, the shorter prerelease precedes.
4. Leading zeros of numeric components and identifiers, and build metadata, do not affect
   precedence.

> **Example:** `1.0-alpha` < `1.0-alpha.1` < `1.0-alpha.beta` < `1.0-beta` < `1.0-beta.2` <
> `1.0-beta.11` < `1.0-rc.1` < `1.0` < `1.0.1` < `1.10`.

### 6.3 Version Requirements

A _version requirement_ is either `*`, which matches every version, or a comma-separated list of
comparators, which matches a version if all comparators do. A comparator consists of an optional
operator and a version, optionally followed by the wildcard `.*`:

```text
requirement = ws ( "*" | comparator { "," comparator } ) ws
comparator  = ws [ operator ws ] version [ ws ".*" ] ws
operator    = "==" | "=" | "!=" | ">=" | ">" | "<=" | "<"
ws          = { whitespace }
```

The operator `=` is an alias for `==`, and a comparator without an operator means `==`. A
comparator with a wildcard MUST use `=`, `==`, or `!=`, and its version MUST NOT have a prerelease
or build metadata. Other requirements are invalid, for example, an empty requirement, a list with
an empty comparator, and a list that contains `*`.

A comparator's version without a prerelease names a _line_: the set of versions whose release
components, with missing components counted as zero, start with the release components of the
comparator's version, including versions with prereleases. The line `3` contains `3.0-rc.1`, `3`,
and `3.5.1`. The line `3.0` contains `3.0.5`, but not `3.1`.

A version is _before_ the line `V` if its first _n_ release components, with missing components
counted as zero, are less than those of `V`, where _n_ is the number of release components of `V`,
and _after_ the line if they are greater. Release components compare as in
[Section 6.2](#62-precedence). A comparator with the version `V` matches a version as follows:

- `==V` matches the versions with the same precedence as `V`, and `!=V` the others.
- `==V.*` matches the versions in the line `V`, and `!=V.*` the others.
- If `V` has a prerelease, `>=V`, `>V`, `<=V`, and `<V` compare the precedence of the version with
  that of `V`.
- Otherwise, they compare lines: `>=V` matches the versions in or after the line `V`, `>V` those
  after it, `<=V` those in or before it, and `<V` those before it.

> **Example:**
>
> - `>=3,<4` matches `3.0-rc.1` and `3.9.2`, but not `2.9` or `4.0-rc.1`.
> - `>3` matches `4.0-rc.1`, but not `3.1`.
> - `>3.0` matches `3.1`, but not `3.0.5`.
> - `<=3` matches `3.9`, but not `4.0-rc.1`.
> - `3.*` matches `3.0-rc.1` and `3.5.1`, but not `4.0-rc.1`.
> - `==3` matches `3.0` and `3+build.7`, but not `3.0-rc.1` or `3.0.1`.
> - `>=3-rc.2` matches `3-rc.2`, `3`, and `4`, but not `3-rc.1`.
> - `<4-rc.2` matches `4-rc.1`, but not `4-rc.2` or `4`.

> **Note:** Lines let `>=3,<4` cover the release candidates of version 3 and exclude those of
> version 4. Package managers such as Cargo exclude prereleases instead, because they choose which
> versions to install; requirements here check versions that are already present.

## 7. Components

The components of a system form its _composition_. Each component declares what it provides and
what it requires, and a compatibility checker determines whether the composition is _consistent_.

### 7.1 Component Declarations

Each component is described by one component declaration:

```sidex
/// One part of a system, such as the system image, an app, a runtime, or a hardware module.
record ComponentDeclaration {
    /// Version of the specification that the record follows; omitted means `1.0`.
    spec_version?: string,
    /// Unique ID of the component within the composition.
    id: string,
    /// Owner of the declaration, which replaces or removes it as a whole.
    owner?: string,
    /// Version of the component, also provided as its implicit capability.
    version?: string,
    /// Capabilities the component provides.
    provides?: [CapabilityDeclaration],
    /// Resources the component claims.
    claims?: [ClaimDeclaration],
    /// Selectors that each need to match a capability.
    requires?: [CapabilitySelectorDeclaration],
    /// Selectors that must not match any capability, including the component's own.
    conflicts?: [CapabilitySelectorDeclaration],
    /// Further data that the specification does not define, with namespaced member names.
    annotations?: [string: json],
}
```

The `version` MUST be valid ([Section 6.1](#61-versions)).

### 7.2 Capabilities

A _capability_ states a fact that a component provides:

```sidex
/// A fact that a component provides.
record CapabilityDeclaration {
    /// ID of the capability.
    id: string,
    /// Version of what is present; mutually exclusive with `value`.
    version?: string,
    /// Exact property; mutually exclusive with `version`.
    value?: string,
}
```

- A capability with only an ID states that something is present.
- A capability with a `version` states which version of something is present. The version MUST be
  valid ([Section 6.1](#61-versions)).
- A capability with a `value` states an exact property as a string.

A capability MUST NOT have both a `version` and a `value`.

Every component also provides an _implicit capability_ with its own `id` and, if it declares one,
its `version`.

> **Example:** `{"id": "docker.compose", "version": "2.27.0"}` states which version of Docker
> Compose is present, and `{"id": "host.arch", "value": "aarch64"}` states the CPU architecture.

### 7.3 Selectors and Claims

Requirements and conflicts are _selectors_ over capabilities:

```sidex
/// A selector over capabilities.
record CapabilitySelectorDeclaration {
    /// ID of the capability.
    id: string,
    /// Version requirement; mutually exclusive with `value`.
    version?: string,
    /// Exact value; mutually exclusive with `version`.
    value?: string,
}
```

The `version` of a selector holds a version requirement ([Section 6.3](#63-version-requirements)),
which MUST be valid. A selector MUST NOT have both a `version` and a `value`. A selector matches a
capability with the same ID if:

- the selector has neither a version requirement nor a value,
- the selector has a version requirement and the capability has a version that matches it, or
- the selector has a value and the capability has exactly that value.

A selector with a version requirement does not match a capability without a version, even if the
requirement is `*`.

A _claim_ reserves a resource:

```sidex
/// A resource that at most one component may claim.
record ClaimDeclaration {
    /// ID of the resource.
    id: string,
}
```

A component that claims the same resource several times claims it once. Claims do not provide
capabilities.

> **Note:** Each requirement is met on its own, possibly by different components, so a component
> that needs two properties of the same provider requires one capability that states both. A
> component that works with `libfoo` only from version 2 on, if it is present at all, declares the
> conflict `{"id": "libfoo", "version": "<2"}`. Alternatives, such as two database engines, provide
> a common capability, such as `database.sql`. IDs of capabilities and claims need shared
> conventions, such as `network.tcp.8080` for a TCP port.

### 7.4 Consistency

A compatibility checker evaluates a composition, which is a list of component declarations, together
with the diagnostics about component declarations and directory configurations. It skips invalid
declarations ([Section 5](#5-diagnostics)), such as declarations with an invalid version, and
considers every explicit and implicit capability of the remaining components. The composition is
_consistent_ if and only if:

- there are no diagnostics with the severity `Error`,
- no two components have the same ID,
- no two components claim the same resource,
- every requirement of every component matches at least one capability, and
- no conflict of any component matches any capability, including the capabilities of the
  component declaring the conflict.

Several components MAY provide the same capability.

### 7.5 Compatibility Reports

A compatibility checker produces a compatibility report:

```sidex
/// Result of checking a composition.
record CompatibilityReport {
    /// Whether there are no problems and no diagnostics with the severity `Error`.
    consistent: bool,
    /// Violated constraints, in the order that the specification defines.
    problems: [CompatibilityProblem],
    /// Diagnostics about component declarations and directory configurations.
    diagnostics: [Diagnostic],
}

/// A violated constraint.
#[json(tag = "problem")]
variant CompatibilityProblem {
    /// More than one component uses an ID.
    DuplicateComponent: ComponentIdProblem,
    /// More than one component claims a resource.
    DuplicateClaim: ClaimProblem,
    /// A requirement matches no capability.
    UnsatisfiedRequirement: SelectorProblem,
    /// A conflict matches a capability.
    Conflict: ConflictProblem,
}

/// A component ID that more than one component uses.
record ComponentIdProblem {
    /// ID of the components.
    id: string,
}

/// A resource that more than one component claims.
record ClaimProblem {
    /// ID of the resource.
    id: string,
    /// IDs of the claiming components, in declaration order.
    components: [string],
}

/// A requirement that matches no capability.
record SelectorProblem {
    /// ID of the requiring component.
    component: string,
    /// The requirement as declared.
    selector: CapabilitySelectorDeclaration,
}

/// A conflict that matches a capability.
record ConflictProblem {
    /// ID of the component declaring the conflict.
    component: string,
    /// The conflict as declared.
    selector: CapabilitySelectorDeclaration,
    /// ID of the component providing the capability.
    provider: string,
    /// The matching capability as declared.
    capability: CapabilityDeclaration,
}
```

The report lists each component ID that more than one component uses, each resource that more than
one component claims, each requirement that matches no capability, and each pair of a conflict and
a capability it matches. Problems MUST appear in this order:

1. Duplicate components, sorted by ID.
2. Duplicate claims, sorted by resource ID.
3. Unsatisfied requirements and conflicts, in the order of the component declarations. Within a
   component, requirements precede conflicts, each in declaration order. A conflict that matches
   several capabilities produces one problem per capability, in the order of the providing
   components, with each component's implicit capability before its explicit ones.

Problems contain the selectors and capabilities as declared, with the fields that the schema
defines.

> **Example:** An app requires `{"id": "docker.compose", "version": ">=2.20,<3"}`. If an update of
> the system image removes Docker Compose 2, the report of the candidate composition is:
>
> ```json
> {
>   "consistent": false,
>   "problems": [
>     {
>       "problem": "UnsatisfiedRequirement",
>       "component": "telemetry",
>       "selector": { "id": "docker.compose", "version": ">=2.20,<3" }
>     }
>   ],
>   "diagnostics": []
> }
> ```

### 7.6 Candidate Compositions

A _candidate composition_ is the composition that would result from an update. An update replaces
all declarations of one or more owners ([Section 4.3](#43-owners)): it names these owners and
supplies new declarations, each of which MUST have one of them. A compatibility checker skips a new
declaration that does not have a replaced owner and adds a diagnostic with the severity `Error` for
it. An update that installs a new contributor replaces an owner that has no declarations yet, and
an update that removes a contributor supplies no declarations for its owner.

The candidate composition is built from the current composition:

1. Remove the declarations with a replaced owner.
2. Append the new declarations, in the order that the update supplies them.

Its diagnostics are the current diagnostics and those about the new declarations. Because the
remaining declarations keep their requirements, checking the candidate composition finds broken
requirements before the update takes place. A fleet-management server can do so with the
composition of a snapshot ([Section 11.1](#111-snapshots)) before it sends an update.

> **Note:** Declarations without an owner are never removed. This suits facts about the device
> itself, such as the revision of its board. A declaration that is meant to be replaced needs an
> owner; otherwise, its replacement duplicates its ID.

> **Note:** If the current composition is already inconsistent, so is every candidate composition
> that keeps its problems. A consumer can accept a candidate composition whose problems and error
> diagnostics all occur in the current composition, provided that the update adds no diagnostics
> about its own declarations, so that existing problems do not block updates that do not make them
> worse.

## 8. Health

Health describes whether a running system and its functions meet declared criteria. Services run
checks and publish their results, the parts of the system declare how the results combine into the
health of larger functions, and a health evaluator evaluates the results against these
declarations.

### 8.1 Health Answers

Every check and every function has two kinds of criteria: _minimum criteria_, under which it is
still usable, and _desired criteria_, under which it is fully healthy. Health evaluation answers
two questions about each node:

- **Usable:** Are the minimum criteria met?
- **Healthy:** Are the desired criteria met?

Each answer is `Yes`, `No`, or `Unknown`:

```sidex
/// Answer to whether criteria are met.
#[json(tagged = externally)]
variant Answer {
    /// The criteria are met.
    Yes,
    /// The criteria are not met.
    No,
    /// The available evidence does not determine whether the criteria are met.
    Unknown,
}
```

A node that is healthy is usable, and a node that is not usable is not healthy. For display and for
publishing, the answers are summarized as four _health states_:

- `Healthy`: healthy `Yes`.
- `Degraded`: usable `Yes` and healthy `No` or `Unknown`.
- `Unhealthy`: usable `No`.
- `Unknown`: usable `Unknown`.

A status record ([Section 8.4](#84-observed-nodes-and-status-records)) reports one of these states,
which corresponds to the answers `Yes`/`Yes`, `Yes`/`No`, `No`/`No`, or `Unknown`/`Unknown`.

> **Note:** A consumer that makes a decision chooses the answer it requires: healthy `Yes` for the
> desired criteria or usable `Yes` for the minimum criteria. `Unknown` satisfies neither.

### 8.2 Health Graphs

A _health graph_ is a set of health nodes:

```sidex
/// A check or function whose health is evaluated.
record HealthNode {
    /// Version of the specification that the record follows; omitted means `1.0`.
    spec_version?: string,
    /// Unique ID of the node within the graph.
    id: HealthNodeId,
    /// Owner of the node, which replaces or removes it as a whole.
    owner?: string,
    /// Human-readable name of the node.
    label?: string,
    /// IDs of associated components; associations create no dependencies.
    components?: [string],
    /// How the node's answers are determined; omitted means an observed node without constraints.
    rule?: HealthRule,
    /// Aggregate nodes that the node attaches to as an input.
    part_of?: [Attachment],
    /// Further data that the specification does not define, with namespaced member names.
    annotations?: [string: json],
}

/// How a node's answers are determined.
#[json(tag = "type")]
variant HealthRule {
    /// The node takes its answers from a status record.
    Observed: ObservationRule,
    /// The node derives its answers from a metric sample.
    Threshold: ThresholdRule,
    /// The node derives its answers from other nodes.
    Aggregate: AggregationRule,
}
```

[Sections 8.4](#84-observed-nodes-and-status-records), [8.6](#86-threshold-nodes), and
[8.7](#87-aggregate-nodes) define the rules, and [Section 9](#9-links-between-components-and-health)
the associated components.

The inputs of aggregate nodes, including attached inputs, form directed edges from each aggregate
node to its inputs. The graph SHOULD be acyclic; nodes on a cycle are not evaluated
([Section 8.8](#88-evaluation-and-reports)).

Nodes that are no input of any other node are _roots_. Tools can show the roots of a graph as the
overall health of a device. Which aggregate nodes contributors attach to is a convention of the
platform that combines them, like the names of owners ([Section 4.3](#43-owners)).

A consumer that makes a decision, such as whether to commit an update, SHOULD base it on a node that
is defined for that decision and covers what the decision depends on. A consumer that uses a node
that does not exist SHOULD treat it as having the answers `Unknown`/`Unknown`.

> **Example:** An integrator decides that an update can be committed once the network and the
> telemetry app are usable, defines a node for this decision, and configures the update controller
> to require its usable answer `Yes`:
>
> ```json
> {
>   "specVersion": "1.0",
>   "id": "device.commit",
>   "rule": {
>     "type": "Aggregate",
>     "inputs": [{ "node": "system.network" }, { "node": "telemetry" }]
>   }
> }
> ```
>
> Because the node lists its inputs, an input whose records are missing is `Unknown`. An aggregate
> node whose inputs only attach themselves cannot tell that a contributor is missing entirely.

### 8.3 Evidence

Status records and metric samples are _evidence_: results of checks and measurements taken on the
running system. Evidence is only valid for a limited time, so that a check that stops running
cannot keep reporting an old result. Times are measured on the _boot-time clock_ of the observed
system, which starts at zero when the system boots and counts suspended time. Unlike a wall clock,
it does not change when the system adjusts its time.

Status records ([Section 8.4](#84-observed-nodes-and-status-records)) and metric samples
([Section 8.5](#85-metrics)) share the fields of evidence: `bootId`, `observedAtMs`, `maxAgeMs`,
`owner`, `tag`, `message`, `details`, and `annotations`. The `details` explain an observation to
people, such as the output of a check.

On Linux, `bootId` is the UUID in `/proc/sys/kernel/random/boot_id`, without the trailing newline,
and the boot-time clock is `CLOCK_BOOTTIME`. Writers MUST use the same boot-time clock as the
health evaluator and MUST set `observedAtMs` to the time the check or measurement took place, not
the time the record was written.

> **Note:** On Linux, a time namespace can offset `CLOCK_BOOTTIME` for the processes in it. A
> writer in such a namespace needs to correct its observation times by the offset.

A health evaluator evaluates evidence in a _health context_:

```sidex
/// Boot and time in which evidence is evaluated.
record HealthContext {
    /// Current boot ID.
    boot_id: string,
    /// Current boot-time clock value in milliseconds.
    now_ms: u64,
}
```

Evidence is _fresh_ if all of the following hold, checked in this order:

1. It exists. Otherwise, the reason is `Missing`.
2. Its `bootId` equals the context's `bootId`. Otherwise, the reason is `OtherBoot`.
3. If the record it is checked against has an `owner`, its `owner` is equal. Otherwise, the reason
   is `OwnerMismatch`.
4. If the record it is checked against has an `expectedTag`, its `tag` is equal. Otherwise, the
   reason is `TagMismatch`.
5. Its `observedAtMs` is at most `nowMs`. Otherwise, the reason is `Future`.
6. Its age, `nowMs` minus `observedAtMs`, is less than its `maxAgeMs` and less than every
   `maxAgeMs` of the records it is checked against. Otherwise, the reason is `Expired`.

The first condition that does not hold determines the reason. Evidence that is not fresh is not
used, and the node's answers are `Unknown`/`Unknown` with that reason. If evidence is fresh, the
rules of the node's type determine the answers from it, and the node's result carries its `message`
and `details`.

> **Example:** A status record observed at 42000 ms with a `maxAgeMs` of 15000 is fresh until
> just before 57000 ms. If its node declares a `maxAgeMs` of 10000, it is fresh until just before
> 52000 ms.

The boot ID and the age do not show that evidence comes from the current instance of a service.
If an update replaces an app during a boot, the evidence of the previous instance stays fresh
until it expires. A writer whose evidence depends on the instance or configuration that it checks
SHOULD set `tag` to identify it, such as the version of an app, and the node and metric
definitions of the same contributor SHOULD declare the matching `expectedTag`.

### 8.4 Observed Nodes and Status Records

An observed node takes its answers from a status record:

```sidex
/// Constraints on the status records of an observed node.
record ObservationRule {
    /// Upper limit for the age of status records, in milliseconds; must be positive.
    max_age_ms?: u64,
    /// Tag that status records need to carry to be fresh.
    expected_tag?: string,
}

/// Result of a check, published while the system runs.
record HealthStatus {
    /// Version of the specification that the record follows; omitted means `1.0`.
    spec_version?: string,
    /// ID of the observed node.
    id: HealthNodeId,
    /// Result of the check.
    state: HealthState,
    /// ID of the boot during which the check took place.
    boot_id: string,
    /// Boot-time clock value, in milliseconds, when the check took place.
    observed_at_ms: u64,
    /// Duration, in milliseconds, for which the status record is valid; must be positive.
    max_age_ms: u64,
    /// Owner of the status record.
    owner?: string,
    /// Opaque label identifying the configuration or instance that took the evidence.
    tag?: string,
    /// Explanation supplied by the writer.
    message?: string,
    /// Further information about the observation, for display.
    details?: [string: json],
    /// Further data that the specification does not define, with namespaced member names.
    annotations?: [string: json],
}

/// Summary of the answers of a node.
#[json(tagged = externally)]
variant HealthState {
    /// The desired criteria are met.
    Healthy,
    /// The minimum criteria are met; whether the desired criteria are met is `No` or `Unknown`.
    Degraded,
    /// The minimum criteria are not met.
    Unhealthy,
    /// Whether the minimum criteria are met is unknown.
    Unknown,
}
```

Writers MUST NOT publish two status records for the same node. If several status records refer to
an observed node, its answers are `Unknown`/`Unknown`, with the reason `Conflict`, and the evaluator
adds a diagnostic with the severity `Warning`. Status records that refer to no observed node are
ignored.

Otherwise, the status record of an observed node is checked against the node: its owner against
the node's `owner`, its tag against the rule's `expectedTag`, and its age against the rule's
`maxAgeMs`. If the status record is fresh, the node's answers correspond to its `state` as defined
in [Section 8.1](#81-health-answers).

> **Note:** A check reports `Degraded` when it is usable but not fully healthy, for example, a
> disk check that sees little free space. It reports `Unknown` when it cannot reach a conclusion,
> for example, because a dependency of the check itself is unavailable.

> **Note:** A status record cannot express the answers `Yes`/`Unknown` and `Unknown`/`No`, so it
> cannot relay the result of an evaluation. A system that passes on the health of another system,
> such as an attached device, passes on that system's own snapshot ([Section 11](#11-transport)).

### 8.5 Metrics

A _metric_ is a named numeric measurement, such as a CPU temperature or the free space on a
partition. Metrics separate measurements from health policies: a service publishes a measurement,
and any number of threshold nodes ([Section 8.6](#86-threshold-nodes)) evaluate it. A metric keeps
no history; only its current sample counts.

A metric definition declares a metric, and a metric sample is evidence
([Section 8.3](#83-evidence)) of its current value:

```sidex
/// A named numeric measurement.
record MetricDefinition {
    /// Version of the specification that the record follows; omitted means `1.0`.
    spec_version?: string,
    /// Unique ID of the metric.
    id: MetricId,
    /// Owner of the definition, which replaces or removes it as a whole.
    owner?: string,
    /// Human-readable name of the metric.
    label?: string,
    /// IDs of associated components; associations create no dependencies.
    components?: [string],
    /// Unit of all samples and thresholds of the metric, as a UCUM code.
    unit: string,
    /// Upper limit for the age of samples, in milliseconds; must be positive.
    max_age_ms?: u64,
    /// Tag that samples need to carry to be fresh.
    expected_tag?: string,
    /// Further data that the specification does not define, with namespaced member names.
    annotations?: [string: json],
}

/// Current measurement of a metric, published while the system runs.
record MetricSample {
    /// Version of the specification that the record follows; omitted means `1.0`.
    spec_version?: string,
    /// ID of the metric.
    id: MetricId,
    /// Measured value in the metric's unit; must be finite.
    value: f64,
    /// ID of the boot during which the measurement took place.
    boot_id: string,
    /// Boot-time clock value, in milliseconds, when the measurement took place.
    observed_at_ms: u64,
    /// Duration, in milliseconds, for which the sample is valid; must be positive.
    max_age_ms: u64,
    /// Owner of the sample.
    owner?: string,
    /// Opaque label identifying the configuration or instance that took the evidence.
    tag?: string,
    /// Explanation supplied by the writer.
    message?: string,
    /// Further information about the observation, for display.
    details?: [string: json],
    /// Further data that the specification does not define, with namespaced member names.
    annotations?: [string: json],
}
```

A metric ID names one measurement in one unit, because threshold nodes of other contributors
compare its samples with fixed numbers. Writers MUST NOT change the meaning or the unit of a
metric; a different measurement or unit needs a new ID. Writers MUST NOT publish two definitions
or two samples for the same metric. If they do, the answers of threshold nodes over the metric
are unknown ([Section 8.6](#86-threshold-nodes)). Samples without a definition are ignored.

Units are case-sensitive codes of the [Unified Code for Units of Measure][ucum] (UCUM), as in
OpenTelemetry, such as `Cel` for degrees Celsius, `By` for bytes, `s` for seconds, `1` for
fractions, where 1 is 100 percent, `By/s` for data rates, and `{process}` for counts of processes.
Writers MUST use a valid UCUM code and SHOULD use a base unit without a prefix, such as `s` and
`By`, so that thresholds of different contributors compare without conversion. Health evaluation
compares units only as strings ([Section 8.6](#86-threshold-nodes)) and never converts values.
Readers MUST NOT reject a metric because of its unit code.

[ucum]: https://ucum.org/ucum

### 8.6 Threshold Nodes

A threshold node derives its answers by comparing a metric sample with two ranges:

```sidex
/// Ranges that a metric sample is compared with.
record ThresholdRule {
    /// ID of the metric.
    metric: MetricId,
    /// Unit that the ranges assume, as a UCUM code.
    unit?: string,
    /// Range in which the desired criteria are met; a subset of the usable range.
    healthy: MetricRange,
    /// Range in which the minimum criteria are met.
    usable: MetricRange,
    /// Upper limit for the age of samples, in milliseconds; must be positive.
    max_age_ms?: u64,
}

/// A non-empty range; a missing bound leaves the range unbounded on that side.
record MetricRange {
    /// Lower bound.
    lower?: MetricBound,
    /// Upper bound.
    upper?: MetricBound,
}

/// One bound of a range, in the metric's unit.
record MetricBound {
    /// Finite value of the bound.
    value: f64,
    /// Whether the bound itself belongs to the range.
    inclusive: bool,
}
```

The `unit` states the unit that the ranges assume, so that a threshold node does not silently apply
its ranges to a metric whose unit differs. A value lies in a range if it is greater than the lower
bound, or equal to it and the bound is inclusive, and less than the upper bound, or equal to it and
the bound is inclusive.

A range MUST NOT be empty: if it has both bounds, the lower value MUST NOT exceed the upper value,
and if both values are equal, both bounds MUST be inclusive. The healthy range MUST be a subset of
the usable range.

If the metric has no definition, the node's answers are `Unknown`/`Unknown`, with the reason
`Missing`. If the metric has several definitions or several samples, or if the rule has a `unit`
that differs from the definition's unit, the node's answers are `Unknown`/`Unknown`, with the reason
`Conflict`, and the evaluator adds a diagnostic with the severity `Warning`. Otherwise, the metric's
sample is checked against the metric's definition and the rule: its owner against the definition's
`owner`, its tag against the definition's `expectedTag`, and its age against the `maxAgeMs` of both.
If the sample is fresh, the node's answers are:

- `Yes`/`Yes` if the value lies in the healthy range,
- `Yes`/`No` if it lies in the usable range but not in the healthy range, and
- `No`/`No` otherwise.

> **Example:** This threshold node over a CPU temperature is healthy at 75 °C, usable but not
> healthy at 90 °C, and not usable above 90 °C:
>
> ```json
> {
>   "specVersion": "1.0",
>   "id": "system.thermal",
>   "rule": {
>     "type": "Threshold",
>     "metric": "system.cpu-temperature",
>     "unit": "Cel",
>     "healthy": { "upper": { "value": 75, "inclusive": true } },
>     "usable": { "upper": { "value": 90, "inclusive": true } }
>   }
> }
> ```

> **Note:** Threshold nodes evaluate only the current sample. Smoothing a fluctuating measurement,
> for example, by averaging, is the responsibility of the service that publishes it.

### 8.7 Aggregate Nodes

An aggregate node derives its answers from other nodes, its _inputs_:

```sidex
/// Inputs of an aggregate node and the counts they need to meet.
record AggregationRule {
    /// The node's inputs.
    inputs?: [AggregationInput],
    /// Number of counted inputs that need to be usable; omitted means `min_healthy`.
    min_usable?: u64,
    /// Number of counted inputs that need to be healthy; omitted means all counted inputs.
    min_healthy?: u64,
}

/// One input of an aggregate node.
record AggregationInput {
    /// ID of the input node.
    node: HealthNodeId,
    /// How the input affects the node's answers; omitted means `Counted`.
    mode?: InputMode,
}

/// How an input affects the answers of an aggregate node.
#[json(tagged = externally)]
variant InputMode {
    /// The input counts toward `min_usable` and `min_healthy`.
    Counted,
    /// The input counts, and the node is only usable and healthy if the input is.
    Mandatory,
    /// The input does not count; the node is only healthy if the input is healthy.
    Optional,
}
```

The input modes are:

- `Counted`: The input counts toward `minUsable` and `minHealthy`.
- `Mandatory`: The input counts, and the node is only usable if the input is usable and only
  healthy if the input is healthy.
- `Optional`: The input does not count and does not affect the minimum criteria. The node is only
  healthy if the input is healthy.

A node also becomes an input of an aggregate node by attaching itself to it with an entry of its
`partOf` field:

```sidex
/// Attachment of a node to an aggregate node as an input.
record Attachment {
    /// ID of the aggregate node.
    node: HealthNodeId,
    /// Mode of the input; omitted means `Counted`.
    mode?: InputMode,
}
```

If the aggregate node exists, the attaching node is one of its inputs, with the mode of the
attachment. Attached inputs follow the inputs of the rule, in the order of the definitions of the
attaching nodes. Attachments to nodes that do not exist or are no aggregate nodes are ignored, so
that contributors can attach to nodes that are not always present.

If a node is an input more than once, listed or attached, it counts as one input, at the position of
its first occurrence. Its mode is the mode of the first input of the rule that refers to it or, if
the rule does not list it, the mode of its first attachment, so that the aggregate node's own rule
decides how its inputs count. An input of the rule that refers to no node has the answers
`Unknown`/`Unknown`, and the evaluator adds a diagnostic with the severity `Warning` for it; the
input appears in the node's result, but has no result of its own.

Inputs with the modes `Counted` and `Mandatory` are the _counted inputs_.

The answers combine in three-valued logic. For a list of answers, their _conjunction_ is `No` if
any answer is `No`, otherwise `Unknown` if any answer is `Unknown`, and `Yes` otherwise; the
conjunction of no answers is `Yes`. For a count _k_ over a list of answers, the _count answer_ is
`Yes` if at least _k_ answers are `Yes`, `No` if fewer than _k_ answers are `Yes` or `Unknown`, and
`Unknown` otherwise.

The answers of an aggregate node are:

- **Usable:** the conjunction of
  - the count answer for `minUsable` over the usable answers of the counted inputs, and
  - the usable answers of the mandatory inputs.
- **Healthy:** the conjunction of
  - the node's usable answer,
  - the count answer for `minHealthy` over the healthy answers of the counted inputs,
  - the healthy answers of the mandatory inputs, and
  - the healthy answers of the optional inputs.

With the default counts, an aggregate node without counted inputs is always usable, and an
aggregate node without inputs is healthy. A count greater than the number of counted inputs cannot
be met.

> **Example:** A telemetry app needs its database and its connection to a machine controller to
> collect data, and an upload to a cloud service to be fully healthy. While the upload fails, the
> app buffers data locally and remains usable. The app attaches its node to `rugix.apps`, an
> aggregate node that its platform defines for all apps:
>
> ```json
> {
>   "specVersion": "1.0",
>   "id": "telemetry",
>   "owner": "rugix.apps.telemetry",
>   "partOf": [{ "node": "rugix.apps" }],
>   "rule": {
>     "type": "Aggregate",
>     "inputs": [
>       { "node": "telemetry.database" },
>       { "node": "telemetry.machine" },
>       { "node": "telemetry.upload", "mode": "Optional" }
>     ]
>   }
> }
> ```
>
> If the upload's status record expires, the app's answers are `Yes`/`Unknown`. If the database is
> not usable, they are `No`/`No`.

> **Example:** With three interchangeable workers, `"minUsable": 2` keeps a service usable while
> one worker is down, and the omitted `minHealthy` requires all three workers for full health. If
> one worker is not usable and another is unknown, the service's usable answer is `Unknown`,
> because whether two workers are usable depends on the unknown one; its healthy answer is `No`,
> because fewer than three workers can be healthy.

### 8.8 Evaluation and Reports

A health evaluator evaluates the health nodes, status records, metric definitions, and metric
samples in a health context, together with the diagnostics about these records and about directory
configurations ([Section 5](#5-diagnostics)). It proceeds as follows:

1. It skips invalid records, for example, nodes with an empty range or evidence with a `maxAgeMs`
   of zero, and adds a diagnostic with the severity `Error` for each.
2. If several nodes have the same ID, it evaluates none of them. The ID has one result, at the
   position of the first of these nodes, with the answers `Unknown`/`Unknown`, the reason
   `Conflict`, no owner, and no inputs. These nodes do not attach to other nodes, and attachments
   to their ID are ignored. The evaluator adds a diagnostic with the severity `Error` for the ID.
3. It adds the attached inputs of the remaining aggregate nodes
   ([Section 8.7](#87-aggregate-nodes)).
4. It gives every node on a cycle the answers `Unknown`/`Unknown` and the reason `Cycle`. A node is
   on a cycle if it is an input of itself, directly or through other aggregate nodes.
5. It determines the answers of every other node by the rules of its type.

The evaluator produces a health report:

```sidex
/// Result of evaluating health.
record HealthReport {
    /// One result per node ID, in the order of the node definitions.
    nodes: [HealthNodeResult],
    /// Diagnostics about health records and directory configurations, and those added during
    /// evaluation.
    diagnostics: [Diagnostic],
}

/// Answers of one node.
record HealthNodeResult {
    /// ID of the node.
    id: HealthNodeId,
    /// Owner of the node.
    owner?: string,
    /// Whether the minimum criteria are met.
    usable: Answer,
    /// Whether the desired criteria are met.
    healthy: Answer,
    /// IDs of the inputs of an aggregate node; empty for other nodes.
    inputs: [HealthNodeId],
    /// Why the node's evidence was not used or its rule was not applied.
    reason?: Reason,
    /// Message of the fresh evidence.
    message?: string,
    /// Details of the fresh evidence.
    details?: [string: json],
}
```

The reasons are:

- `Missing`, `OtherBoot`, `OwnerMismatch`, `TagMismatch`, `Future`, and `Expired` for evidence
  that is not fresh ([Section 8.3](#83-evidence)), and `Missing` also for a metric without a
  definition,
- `Conflict` for records that contradict each other, such as two nodes with the same ID,
- `Cycle` for a node on a cycle, and
- `Unsupported` for a rule type, state, or input mode unknown to the evaluator
  ([Section 12](#12-evolution)).

Other results of aggregate nodes have no reason; their inputs explain them.

> **Example:** An app's observed node `telemetry` attaches itself to `device.apps`, an aggregate
> node without own inputs. If the app's status record expires, the report is:
>
> ```json
> {
>   "nodes": [
>     { "id": "device.apps", "usable": "Unknown", "healthy": "Unknown", "inputs": ["telemetry"] },
>     {
>       "id": "telemetry",
>       "usable": "Unknown",
>       "healthy": "Unknown",
>       "inputs": [],
>       "reason": "Expired"
>     }
>   ],
>   "diagnostics": []
> }
> ```

A skipped node or directory configuration can have provided inputs of other nodes, which then appear
healthier than they are. A consumer that makes a decision, such as whether to commit an update,
SHOULD NOT base it on a report with diagnostics of the severity `Error` and the kinds `HealthNode`
or `DirectoryConfig`, unless it is configured to. Skipped evidence and metric definitions make the
nodes that use them unknown, unless another record with the same ID remains. A consumer can also
require a report without any error diagnostics.

## 9. Links Between Components and Health

The components part and the health part are independent. Compatibility checking does not consider
health, and health evaluation does not consider compatibility. The parts are linked in one
direction only: health nodes and metric definitions MAY name associated components in their
`components` field.

An association states that a node or metric concerns a component, for example, that a database
check concerns an app. It creates no dependency: a health evaluator MUST NOT consider whether
associated components exist, and associated components do not affect a node's answers. Consumers
MAY use associations to present health per component, for example, to show the health nodes that
concern an app.

## 10. Discovery

This section defines how writers publish records as files on Linux and how readers discover them.
Directory configurations ([Section 10.4](#104-directory-configurations)), which add directories, are
published and read like records.

### 10.1 Directories

Records and directory configurations have standard directories below `/usr/lib/system-runtime`,
`/etc/system-runtime`, and `/run/system-runtime`:

- component declarations in `components`,
- health nodes in `health/nodes`,
- metric definitions in `metrics/definitions`, and
- directory configurations in `directories`.

Status records and metric samples only have the standard directories
`/run/system-runtime/health/status` and `/run/system-runtime/metrics/samples`.

> **Note:** `/usr/lib/system-runtime` holds records of the system image, `/etc/system-runtime`
> records specific to a device, and `/run/system-runtime` records generated at runtime. Evidence
> only has directories below `/run`, because it is only valid during the boot in which it was
> taken.

The directories form three _levels_. From highest to lowest, they are the directories below `/etc`,
the directories below `/run`, and the directories below `/usr/lib` together with the directories
added by directory configurations ([Section 10.4](#104-directory-configurations)).

### 10.2 Files

Each record is a regular file with the extension `.json` that contains exactly one record of the
kind of its directory. The file name has no meaning.

A writer MUST write a file so that readers never see a partially written file. It SHOULD write the
record to a temporary file in the same directory, whose name starts with `.` or has another
extension, and then rename the temporary file to the final name. There are no transactions across
files.

> **Note:** To let an unprivileged service publish its evidence without giving it access to the
> files of other services, give the service a directory of its own and add the directory with a
> directory configuration. This separates files, but does not restrict the IDs that the service can
> publish.

### 10.3 Reading

To read the records of one kind, a reader first reads the directory configurations. It then visits
the standard directories in the order `/usr/lib`, `/etc`, `/run`, followed by the directories that
the configurations add for that kind, in the order in which it read the configurations. It traverses
each directory depth-first, visiting the entries of a directory in the order of the bytes of their
names and descending into a subdirectory when it visits it. While doing so, the reader:

1. skips entries whose names start with `.`, and symlinks within a directory, whether they point
   to files or to directories,
2. reads every regular file with the extension `.json` and skips other files, and
3. reads each file at most once: a file whose canonical absolute path it has already read is
   skipped, even if another directory contains it. Different paths to the same file, such as hard
   links, count as different files.

A reader MUST skip directories that do not exist, and entries that disappear between listing a
directory and reading them. If a directory path exists but is not a directory or is a broken
symlink, if a directory cannot be listed, or if a file cannot be read or does not contain a valid
record, a reader MUST skip it and report a diagnostic with the severity `Error`
([Section 5](#5-diagnostics)). A path whose existence cannot be determined is reported in the
same way.

Each record has the level of the directory through which the reader read it. A file that the reader
reaches through directories of several levels keeps the position of its first visit and has the
highest of these levels. Component declarations, health nodes, and metric definitions override
records of the same kind with the same ID and the same owner from lower levels: of these records, a
reader keeps only those from the highest level, at their own positions. Two records have the same
owner if both have none or both have equal owners. Records with the same ID and different owners do
not override each other; like evidence with the same ID, they are subject to the rules of their
part. Invalid records do not override other records.

> **Note:** An override carries the owner of the record it overrides, so the next update of that
> owner removes it ([Section 4.3](#43-owners)) and it cannot outlive the record it changes. Whoever
> manages a device's overrides, such as a fleet-management tool, applies them again after updates.
> To change a threshold, an operator copies its node to `/etc/system-runtime/health/nodes` and
> changes the ranges. To disable a check, an operator overrides its node with an aggregate node
> without inputs, such as `{"id": "telemetry.upload", "owner": "rugix.apps.telemetry", "rule":
> {"type": "Aggregate"}}`, which is healthy as long as no other node attaches to it.

A directory MAY be a symlink. A reader MUST resolve each directory to its physical location before
traversing it, so that changing the symlink during traversal cannot mix files of two
directories.

### 10.4 Directory Configurations

A directory configuration adds directories for other contributors, such as apps that ship records
in their installation directories. It is encoded, validated, and versioned like a record
([Sections 4.1](#41-encoding) and [4.5](#45-specification-versions)):

```sidex
/// Directories that a contributor adds, as absolute paths without `..` segments.
///
/// A segment consisting only of `*` matches every directory at that level whose name does not
/// start with `.`, in byte order, but no symlink.
record DirectoryConfig {
    /// Version of the specification that the record follows; omitted means `1.0`.
    spec_version?: string,
    /// Directories of component declarations.
    components?: [string],
    /// Directories of health nodes.
    health_nodes?: [string],
    /// Directories of status records.
    health_statuses?: [string],
    /// Directories of metric definitions.
    metrics?: [string],
    /// Directories of metric samples.
    metric_samples?: [string],
}
```

Paths MUST be absolute and MUST NOT contain `..` segments or NUL characters. A path segment
consisting only of `*` matches every directory at that level whose name does not start with `.`, in
the order of the bytes of their names, but no symlink. Other wildcards, such as `*.d` or `**`, are
invalid. Readers MUST expand wildcards whenever they read records. Directory configurations cannot
add directories for directory configurations.

> **Example:** This configuration discovers the records of all installed apps of an app manager
> that keeps the active version of each app behind a `current` symlink:
>
> ```json
> {
>   "components": ["/var/lib/apps/*/current/components"],
>   "healthNodes": ["/var/lib/apps/*/current/health/nodes"]
> }
> ```

## 11. Transport

This section defines how records are sent to another system, such as a fleet-management server,
so that it can check compatibility or evaluate health. Snapshots and reports use the encoding of
[Section 4.1](#41-encoding). A snapshot does not identify the system it describes; the transport
does.

### 11.1 Snapshots

A snapshot contains the records read from a system:

```sidex
/// Records read from a system, for sending them to another system.
///
/// An omitted list means that the snapshot does not include records of its kind. The lists of the
/// health part are either all present or all omitted.
record RuntimeSnapshot {
    /// Version of the specification that the snapshot's own fields follow.
    spec_version: string,
    /// Correlation of the boot-time clock with wall-clock time, sampled after collecting the
    /// records.
    clock: HealthClockSample,
    /// Component declarations.
    components?: [ComponentDeclaration],
    /// Health nodes.
    health_nodes?: [HealthNode],
    /// Status records.
    health_statuses?: [HealthStatus],
    /// Metric definitions.
    metrics?: [MetricDefinition],
    /// Metric samples.
    metric_samples?: [MetricSample],
    /// Diagnostics about the included records and about directory configurations.
    diagnostics?: [Diagnostic],
}
```

The writer of a snapshot MUST include all records of each included kind that its reader read,
after overrides ([Section 10.3](#103-reading)), and MUST include either all lists of the health part
or none. It MUST include the diagnostics of both severities that its reader reported about the
included kinds of records and about directory configurations, and omits `diagnostics` if there are
none.

A snapshot MUST contain the original records. A system that relays a snapshot is a writer of it
and MUST NOT change it, in particular not the observation times or validity durations.

The records in a snapshot keep their own `specVersion`. A reader MUST NOT interpret a snapshot
unless it supports the snapshot's `specVersion`, which it does for all versions of its own major
version; it applies the fallbacks of [Section 12](#12-evolution) to newer minor versions.

### 11.2 Clock Correlation

The reader of a snapshot cannot interpret boot-time clock values on its own. A clock sample
correlates them with wall-clock time:

```sidex
/// Correlation of the boot-time clock with wall-clock time.
record HealthClockSample {
    /// Boot ID and boot-time clock value at sampling.
    context: HealthContext,
    /// Wall-clock time at the same instant, in RFC 3339 format in UTC.
    timestamp: timestamp,
}
```

The writer of a snapshot MUST take the clock sample after collecting the records. Calculations with
timestamps ignore leap seconds, as Unix time does.

Evaluating a snapshot with the context of its clock sample evaluates the collected evidence at the
time of the clock sample. For evidence from the boot of the clock sample, its estimated wall-clock
time is the clock sample's `timestamp`, truncated to milliseconds, minus `nowMs − observedAtMs`
milliseconds. Evidence from another boot, evidence with an observation time after the clock sample,
and evidence whose estimate lies outside the years 0001 to 9999 have no estimated wall-clock time.

> **Example:** With the clock sample `{"context": {"bootId": "…", "nowMs": 45000}, "timestamp":
> "2026-10-14T12:00:00Z"}`, a status record observed at 42000 ms was observed at about
> `2026-10-14T11:59:57Z`. The estimate is only as accurate as the wall clock at the time of
> sampling.

## 12. Evolution

The specification is versioned with `MAJOR.MINOR` version numbers. A minor version only makes
changes that readers of earlier minor versions of the same major version can safely ignore:

- adding optional fields whose absence keeps the meaning of a record unchanged, and
- adding values to the enumerations that this section lists, with the fallback defined here.

A component declaration whose `specVersion` ([Section 4.5](#45-specification-versions)) has a higher
minor version than the reader supports is invalid, because a matching feature that the reader does
not know could make it match too much. For other records with a higher minor version, the reader
uses them, applying the fallbacks of this section, and reports a diagnostic with the severity
`Warning`.

The schemas describe what writers produce. Implementations MUST ignore fields they do not know,
and MUST treat values they do not know as follows:

- **Health evaluators** treat an unknown rule type, or an unknown state of a fresh status record,
  as unsupported: the node's answers are `Unknown`/`Unknown`, with the reason `Unsupported`. An
  unknown input mode makes the aggregate node's answers unsupported in the same way.
- **Consumers** report an unknown compatibility problem without further detail and treat an
  unknown reason as no reason.
- **Readers of snapshots and consumers** treat a diagnostic of an unknown kind as of the kind
  `DirectoryConfig`, and one of an unknown severity as an error.

A change that readers cannot safely ignore requires a new major version. A new major version uses
new standard directories ([Section 10.1](#101-directories)), so that older readers do not report its
records as invalid.

## 13. Security Considerations

Records decide whether updates are installed, committed, or rolled back, so false records can block
updates or get faulty ones committed.

**Trust.** Readers trust every writer that can create files in the directories they read, with any
ID and owner. Such a writer can attach nodes to any aggregate node, conflict with the records of
others, and, from a higher level, override the records of the system image. Owners and tags detect
mistakes, not attacks. Standard directories and added directories should therefore be writable only
by privileged users and by the services that publish into them.

**Resource limits.** Readers SHOULD limit the size of files, the number of files, the depth of
directories, and the nesting of JSON values, and SHOULD skip what exceeds a limit and report a
diagnostic.

**Concurrent changes.** A writer can replace a file or a directory with a symlink after a reader
has checked it. Readers SHOULD traverse directories relative to open file descriptors and open
entries without following symlinks, for example, with `openat` and `O_NOFOLLOW` on Linux.

**Transport.** Snapshots reveal information about a device. Writers SHOULD NOT put secrets into
records. Systems that send snapshots SHOULD protect their integrity and confidentiality and
authenticate the sending device, for example, with TLS.

**Display.** Consumers SHOULD treat labels, messages, and details as untrusted text when they
display them.
