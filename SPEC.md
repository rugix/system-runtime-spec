# System Runtime Specification

Version 0.2

The System Runtime Specification defines how the parts of a system describe two things: which
components they consist of, so that tools can check whether the components are compatible with
each other, and how healthy they are while the system runs, so that tools can evaluate the health
of the system and its functions. A _system_ is everything that makes up a device, such as its
system image, its apps, and its hardware modules. The parts publish small records, and tools such
as update controllers, diagnostic tools, and fleet-management agents read and evaluate them.

This document is the normative definition of the specification. The
[README](README.md) gives an overview and links to implementations.

## Contents

1. [Conventions](#1-conventions)
2. [Conformance](#2-conformance)
3. [Records](#3-records)
4. [Versions and Version Requirements](#4-versions-and-version-requirements)
5. [Components](#5-components)
6. [Health](#6-health)
7. [Links Between Components and Health](#7-links-between-components-and-health)
8. [Discovery](#8-discovery)
9. [Transport](#9-transport)

## 1. Conventions

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY, and
OPTIONAL in this document are to be interpreted as described in [BCP 14][bcp14] (RFC 2119 and
RFC 8174) when, and only when, they appear in all capitals.

Examples and notes are informative. Everything else is normative.

The [schemas](schemas/) define the structure of all records in [Sidex][sidex], a language for
defining data models. They are normative for the fields of records and their types. The
[JSON Schemas](json-schema/) are generated from them. Where this document describes a record, it
lists its fields; the schemas give the same information in machine-readable form.

[bcp14]: https://www.rfc-editor.org/info/bcp14
[sidex]: https://github.com/silitics/sidex

## 2. Conformance

The specification consists of a foundation and two parts that build on it:

- The **foundation** covers records, versions, discovery, and transport ([Sections 3](#3-records),
  [4](#4-versions-and-version-requirements), [8](#8-discovery), and [9](#9-transport)).
- The **components part** covers component declarations and compatibility checking
  ([Section 5](#5-components)).
- The **health part** covers health graphs, status records, metrics, and health evaluation
  ([Section 6](#6-health)).

The two parts are independent. An implementation MAY implement either part or both, together
with the foundation. [Section 7](#7-links-between-components-and-health) defines how records of
the two parts refer to each other.

The specification distinguishes four roles:

- A **writer** publishes records, such as a system image that ships its component declaration or a
  service that publishes the results of its checks.
- A **reader** discovers and parses records ([Section 8](#8-discovery)).
- A **compatibility checker** evaluates component declarations
  ([Section 5](#5-components)).
- A **health evaluator** evaluates health graphs ([Section 6](#6-health)).

An implementation conforms to the specification for a role if it satisfies all requirements of
this document that apply to that role. The [conformance suite](tests/fixtures/) in this repository
contains test cases for compatibility checkers, health evaluators, and version matching. Passing
it is evidence of conformance, but does not replace the requirements of this document. If a test
case contradicts this document, the document applies, and the test case is wrong.

## 3. Records

### 3.1 Encoding

Records are JSON objects. Field names use camelCase. Writers MUST omit optional fields that are
not set and MUST NOT encode them as `null`.

Unsigned 64-bit integers, such as boot-time milliseconds, are encoded as JSON numbers if they are
at most 2⁵³ − 1 and as decimal strings otherwise, so that JavaScript implementations can process
them without losing precision. Readers MUST accept both encodings.

### 3.2 Identifiers

Records are identified by IDs, which are opaque, case-sensitive strings compared by exact
equality. Each kind of record has its own namespace: a component, a health node, and a metric MAY
share an ID without referring to each other.

### 3.3 Owners

Component declarations, health nodes, status records, metric definitions, and metric samples have
an optional `owner`, an opaque, case-sensitive string. The owner names the contributor that
supplies the record, such as a system image or an app, and that replaces or removes the record as
a whole.

Owners do not scope IDs: records with different owners MUST still have distinct IDs within their
namespace. Records without an owner are valid. [Section 5.6](#56-candidate-compositions) defines
how owners determine which records an update replaces, and
[Section 6.3](#63-evidence) how they constrain evidence.

> **Note:** A system image might use the owner `rugix.system`, and an app named `telemetry` the
> owner `rugix.apps.telemetry`. Owner names are conventions of the tools that use them; readers do
> not derive owners from file paths.

### 3.4 Evolution

The specification is versioned with `MAJOR.MINOR` version numbers. Before version 1.0, a minor
version MAY change the meaning of existing records. From version 1.0 on, a minor version only
makes changes that readers of earlier minor versions of the same major version can safely
ignore:

- adding optional fields whose absence keeps the meaning of a record unchanged, and
- adding values to the enumerations that this section lists, with the fallback defined here.

Readers MUST ignore fields they do not know. Readers MUST treat values they do not know as
follows:

| Enumeration | Fallback |
| --- | --- |
| Health rule types ([Section 6.2](#62-health-graphs)) | The node's result is unknown, with the reason `unsupported`. |
| Status states ([Section 6.4](#64-observed-nodes-and-status-records)) | The node's result is unknown, with the reason `unsupported`. |
| Metric units ([Section 6.5](#65-metrics)) | Threshold nodes referencing the metric evaluate as unknown, with the reason `unsupported`, if the sample is fresh. |
| Compatibility problems ([Section 5.5](#55-compatibility-reports)) | The problem is reported without further detail. |
| Reasons ([Section 6.7](#67-evaluation-and-reports)) | The result is treated as having no reason. |

From version 1.0 on, a change that readers cannot safely ignore requires a new major version. A new major version MUST
use new standard directories ([Section 8.1](#81-directories)), so that readers of an earlier major
version never misinterpret its records.

## 4. Versions and Version Requirements

Components declare versions, and other components declare requirements on them. Because the parts
of a system follow different versioning schemes, such as Semantic Versioning for libraries and
Calendar Versioning for system images, versions use one grammar that covers these schemes without
knowing which one applies.

### 4.1 Versions

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
components and prerelease identifiers that consist only of digits are _numeric_ and MUST fit into
an unsigned 64-bit integer. Leading zeros are allowed. Readers MUST ignore whitespace at the start
and end of a version. In this document, _whitespace_ means the characters with the Unicode property
`White_Space`.

> **Note:** The grammar covers, for example, `1.4.2`, `2.0.0-rc.1`, `1.4.2+build.7`, `2026.06.25`,
> `42`, and `20260625071935`. Versions of other formats must be converted. For example, a Debian
> version such as `1:2.27~rc1-3` could become `2.27-rc1+3`, if all compared versions share the
> epoch `1`.

### 4.2 Precedence

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

### 4.3 Version Requirements

A _version requirement_ is either `*`, which matches every version, or a comma-separated list of
comparators, all of which MUST match. A comparator consists of an optional operator and a version,
optionally followed by the wildcard `.*`. The operators are `==`, `!=`, `>`, `>=`, `<`, and `<=`.
The operator `=` is an alias for `==`, and a comparator without an operator means `==`. Readers
MUST ignore whitespace at the start and end of a requirement and of each comparator, and between an
operator and its version.

A comparator with a wildcard MUST use `=`, `==`, or `!=`, and its version MUST NOT have a
prerelease or build metadata. Other requirements are invalid.

A comparator's version without a prerelease names a _line_: the set of versions whose release
components, with missing components counted as zero, start with the release components of the
comparator's version, including versions with prereleases. The line `3` contains `3.0-rc.1`, `3`,
and `3.5.1`. The line `3.0` contains `3.0.5`, but not `3.1`.

A comparator matches a version as follows, where `V` is the comparator's version:

| Comparator | `V` without a prerelease | `V` with a prerelease |
| --- | --- | --- |
| `>=V` | The release components of the version are at least those of `V`, so the version is in or after the line `V`. | The version's precedence is at least that of `V`. |
| `<V` | The release components of the version are less than those of `V`, so the version is before the line `V`. | The version's precedence is less than that of `V`. |
| `>V` | The version is after the line `V`. | The version's precedence is greater than that of `V`. |
| `<=V` | The version is in or before the line `V`. | The version's precedence is at most that of `V`. |
| `==V` | The version has the same precedence as `V`. | The version has the same precedence as `V`. |
| `!=V` | The version has a different precedence than `V`. | The version has a different precedence than `V`. |
| `==V.*` | The version is in the line `V`. | Invalid. |
| `!=V.*` | The version is not in the line `V`. | Invalid. |

Release components compare as in [Section 4.2](#42-precedence), ignoring prereleases. A version is
after the line `V` if its first _n_ release components, with missing components counted as zero,
are greater than those of `V`, where _n_ is the number of release components of `V`. For example:

| Requirement | Matches | Does Not Match |
| --- | --- | --- |
| `>=3,<4` | `3.0-rc.1`, `3.9.2` | `2.9`, `4.0-rc.1` |
| `>3` | `4.0-rc.1` | `3.1` |
| `>3.0` | `3.1` | `3.0.5` |
| `<=3` | `3.9` | `4.0-rc.1` |
| `3.*` | `3.0-rc.1`, `3.5.1` | `4.0-rc.1` |
| `==3` | `3.0`, `3+build.7` | `3.0-rc.1`, `3.0.1` |
| `>=3-rc.2` | `3-rc.2`, `3`, `4` | `3-rc.1` |
| `<4-rc.2` | `4-rc.1` | `4-rc.2`, `4` |

> **Note:** Lines let a requirement such as `>=3,<4` cover the release candidates of version 3,
> which is useful while version 3 is being tested, and exclude those of version 4, which already
> contain the changes of version 4.

## 5. Components

A _component_ is one part of a system, such as the system image, an app, a runtime, or a hardware
module. Together, the components of a system form its _composition_. Components declare what they
provide and require, and a compatibility checker determines whether the composition is
_consistent_.

### 5.1 Component Declarations

Each component is described by one `ComponentDeclaration`:

| Field | Type | Description |
| --- | --- | --- |
| `id` | string | Unique ID of the component within the composition. |
| `owner` | string, optional | Owner of the declaration ([Section 3.3](#33-owners)). |
| `version` | version, optional | Version of the component. |
| `provides` | list of capabilities, optional | Capabilities the component provides. |
| `claims` | list of claims, optional | Resources the component claims. |
| `requires` | list of selectors, optional | Requirements: selectors that each need to match a capability. |
| `conflicts` | list of selectors, optional | Conflicts: selectors that must not match any capability. |

### 5.2 Capabilities

A _capability_ states a fact that a component provides. It has an `id` and, optionally, either a
`version` or a `value`, but not both:

- A capability with only an ID states that something is present.
- A capability with a `version` states which version of something is present. The version MUST be
  valid ([Section 4.1](#41-versions)).
- A capability with a `value` states an exact property as a string.

Every component also provides an _implicit capability_ with its own `id` and, if it declares one,
its `version`.

> **Example:** `{"id": "docker.compose", "version": "2.27.0"}` states which version of Docker
> Compose is present, and `{"id": "host.arch", "value": "aarch64"}` states the CPU architecture.

### 5.3 Selectors and Claims

Requirements and conflicts are _selectors_ over capabilities. A selector has an `id` and,
optionally, either a `version` holding a version requirement ([Section 4.3](#43-version-requirements))
or a `value`, but not both. A selector matches a capability with the same ID if:

- the selector has neither a version requirement nor a value,
- the selector has a version requirement and the capability has a version that matches it, or
- the selector has a value and the capability has exactly that value.

A selector with a version requirement does not match a capability without a version, even if the
requirement is `*`.

A _claim_ reserves a resource that at most one component may claim. It has an `id` that names the
resource. A component that claims the same resource several times claims it once. Claims do not
provide capabilities.

> **Note:** Claims only collide if all components name a resource the same way, so claim IDs need
> shared conventions, such as `network.tcp.8080` for a TCP port.

### 5.4 Consistency

A compatibility checker evaluates a composition, which is a list of component declarations. It
considers every explicit and implicit capability of every component. The composition is
_consistent_ if and only if:

- no two components have the same ID,
- no two components claim the same resource,
- every requirement of every component matches at least one capability, and
- no conflict of any component matches any capability, including the capabilities of the
  component declaring the conflict.

Several components MAY provide the same capability.

A composition is invalid if a declaration contains an invalid version or version requirement, or a
capability or selector with both a version and a value. For an invalid composition, a
compatibility checker MUST report an error and MUST NOT produce a compatibility report.

### 5.5 Compatibility Reports

A compatibility checker produces a `CompatibilityReport` with the fields `consistent`, a boolean,
and `problems`, a list of all violated constraints. Each problem has a `problem` field naming its
kind:

| Kind | Fields | Reported for |
| --- | --- | --- |
| `duplicateComponent` | `id` | Each component ID that more than one component uses. |
| `duplicateClaim` | `id`, `components` | Each resource that more than one component claims, with the claiming components in declaration order. |
| `unsatisfiedRequirement` | `component`, `selector` | Each requirement that matches no capability. |
| `conflict` | `component`, `selector`, `provider`, `capability` | Each pair of a conflict and a capability it matches. |

The report is consistent if and only if it contains no problems. Problems MUST appear in this
order:

1. Duplicate components, sorted by ID.
2. Duplicate claims, sorted by resource ID.
3. Unsatisfied requirements and conflicts, in the order of the component declarations. Within a
   component, requirements precede conflicts, each in declaration order. A conflict that matches
   several capabilities produces one problem per capability, in the order of the providing
   components, with each component's implicit capability before its explicit ones.

IDs are sorted by Unicode scalar values.

### 5.6 Candidate Compositions

A _candidate composition_ is the composition that would result from replacing the declarations of
one owner, for example, when an update replaces a system image or an app. It is built from the
current composition as follows:

1. Remove all declarations with the owner being replaced.
2. Add the declarations of the replacement.
3. Keep all other declarations unchanged.

Checking the candidate composition finds broken requirements before the replacement takes place,
because the remaining declarations keep their requirements.

> **Note:** Declarations without an owner are never removed. This suits facts about the device
> itself, such as the revision of its board. A declaration that is meant to be replaced needs an
> owner; otherwise, its replacement duplicates its ID.

## 6. Health

Health describes whether a running system and its functions meet declared criteria. Services run
checks and publish their results, the parts of the system declare how the results combine into the
health of larger functions, and a health evaluator evaluates the results against these
declarations.

### 6.1 Health Answers

Every check and every function has two kinds of criteria: _minimum criteria_, under which it is
still usable, and _desired criteria_, under which it is fully healthy. Health evaluation answers
two questions about each node:

- **Usable:** Are the minimum criteria met?
- **Healthy:** Are the desired criteria met?

Each answer is `yes`, `no`, or `unknown`. The answer `unknown` means that the available evidence
does not determine whether the criteria are met. The answers are related: a node that is healthy
is usable, and a node that is not usable is not healthy. Therefore, exactly six combinations are
possible:

| Usable | Healthy | Meaning |
| --- | --- | --- |
| `yes` | `yes` | The desired criteria are met. |
| `yes` | `no` | The node is usable, but the desired criteria are not met. |
| `yes` | `unknown` | The node is usable, but whether the desired criteria are met is unknown. |
| `unknown` | `no` | The desired criteria are not met, and whether the node is usable is unknown. |
| `unknown` | `unknown` | Whether the node is usable is unknown. |
| `no` | `no` | The node is not usable. |

For display and for publishing, the combinations are summarized as four _health states_:

| State | Answers |
| --- | --- |
| `healthy` | Healthy `yes`. |
| `degraded` | Usable `yes` and healthy `no` or `unknown`. |
| `unhealthy` | Usable `no`. |
| `unknown` | Usable `unknown`. |

A status record ([Section 6.4](#64-observed-nodes-and-status-records)) reports one of these states,
which corresponds to the answers `yes`/`yes`, `yes`/`no`, `no`/`no`, or `unknown`/`unknown`.

> **Note:** A tool that requires a node to be fully healthy, such as an update controller
> deciding whether to commit an update, requires the healthy answer `yes`. The answer `unknown`
> does not satisfy such a requirement.

### 6.2 Health Graphs

A _health graph_ is a set of health nodes. Each node is described by a `HealthNode`:

| Field | Type | Description |
| --- | --- | --- |
| `id` | string | Unique ID of the node within the graph. |
| `owner` | string, optional | Owner of the node ([Section 3.3](#33-owners)). |
| `label` | string, optional | Human-readable name of the node. |
| `components` | list of strings, optional | IDs of associated components ([Section 7](#7-links-between-components-and-health)). |
| `rule` | rule, optional | How the node's answers are determined. |

A rule has a `type` field and the fields of its type:

- `observed`: The node takes its answers from a status record
  ([Section 6.4](#64-observed-nodes-and-status-records)).
- `threshold`: The node derives its answers from a metric sample
  ([Section 6.6](#66-threshold-nodes)).
- `aggregate`: The node derives its answers from other nodes
  ([Section 6.8](#68-aggregate-nodes)).

A node without a `rule` is an observed node without constraints. A node whose rule type is unknown
to the evaluator has the answers `unknown`/`unknown`, with the reason `unsupported`.

The inputs of aggregate nodes form directed edges from each aggregate node to its inputs. The
graph MUST be acyclic. Nodes that are no input of any other node are _roots_. A graph MAY have
several roots, for example, one for the functions that must be healthy before an update is
committed and one for everything an operator should monitor.

### 6.3 Evidence

Status records and metric samples are _evidence_: results of checks and measurements taken on the
running system. Evidence is only valid for a limited time, so that a check that stops running
cannot keep reporting an old result. Times are measured on the _boot-time clock_ of the observed
system, which starts at zero when the system boots and counts suspended time. Unlike a wall clock,
it does not change when the system adjusts its time.

Evidence has the following fields:

| Field | Type | Description |
| --- | --- | --- |
| `bootId` | string | ID of the boot during which the evidence was taken. |
| `observedAtMs` | unsigned 64-bit integer | Boot-time clock value, in milliseconds, when the evidence was taken. |
| `maxAgeMs` | unsigned 32-bit integer | Duration, in milliseconds, for which the evidence is valid. MUST be positive. |
| `owner` | string, optional | Owner of the evidence. |
| `tag` | string, optional | Opaque label identifying the configuration or instance that took the evidence. |
| `message` | string, optional | Explanation supplied by the writer. |
| `details` | object, optional | Further information supplied by the writer, in any structure. |

On Linux, `bootId` is the UUID in `/proc/sys/kernel/random/boot_id`, without the trailing newline,
and the boot-time clock is `CLOCK_BOOTTIME`. Writers MUST set `observedAtMs` to the time the check or measurement took place,
not the time the record was written.

A health evaluator evaluates evidence in a _health context_, which consists of the current boot ID,
`bootId`, and the current boot-time clock value in milliseconds, `nowMs`. Evidence is _fresh_ if
all of the following hold, checked in this order:

1. It exists. Otherwise, the reason is `missing`.
2. Its `bootId` equals the context's `bootId`. Otherwise, the reason is `otherBoot`.
3. If the record it is checked against has an `owner`, its `owner` is equal. Otherwise, the reason
   is `ownerMismatch`.
4. If the record it is checked against has an `expectedTag`, its `tag` is equal. Otherwise, the
   reason is `tagMismatch`.
5. Its `observedAtMs` is at most `nowMs`. Otherwise, the reason is `future`.
6. Its age, `nowMs` minus `observedAtMs`, is less than its `maxAgeMs` and less than every
   `maxAgeMs` of the records it is checked against. Otherwise, the reason is `expired`.

The first condition that does not hold determines the reason. Evidence that is not fresh is not
used, and the node's answers are `unknown`/`unknown` with that reason. Only fresh evidence is
interpreted: if it is fresh, the node's result carries its `message` and `details`, and the rules of
the node's type determine the answers.

> **Example:** A status record observed at 42000 ms with a `maxAgeMs` of 15000 is fresh until
> just before 57000 ms. If its node declares a `maxAgeMs` of 10000, it is fresh until just before
> 52000 ms.

### 6.4 Observed Nodes and Status Records

An observed node takes its answers from a status record. Its rule has the following optional
fields:

| Field | Type | Description |
| --- | --- | --- |
| `maxAgeMs` | unsigned 32-bit integer, optional | Upper limit for the age of status records. MUST be positive. |
| `expectedTag` | string, optional | Tag that status records need to carry to be fresh. |

A `HealthStatus` is evidence ([Section 6.3](#63-evidence)) with the following further fields:

| Field | Type | Description |
| --- | --- | --- |
| `id` | string | ID of the observed node. |
| `state` | health state | Result of the check: `healthy`, `degraded`, `unhealthy`, or `unknown`. |

A status record is checked against its node: its owner against the node's `owner`, its tag against
the rule's `expectedTag`, and its age against the rule's `maxAgeMs`. If the status record is fresh,
the node's answers correspond to its `state` as defined in [Section 6.1](#61-health-answers). If
the state is unknown to the evaluator, the node's answers are `unknown`/`unknown`, with the reason
`unsupported`.

There MUST be at most one status record per node. Status records MUST NOT refer to threshold or
aggregate nodes. Status records that refer to no node in the graph are ignored, but MUST still be
valid.

> **Note:** A check reports `degraded` when it is usable but not fully healthy, for example, a
> disk check that sees little free space. It reports `unknown` when it cannot reach a conclusion,
> for example, because a dependency of the check itself is unavailable.

### 6.5 Metrics

A _metric_ is a named numeric measurement, such as a CPU temperature or the free space on a
partition. Metrics separate measurements from health policies: a service publishes a measurement,
and any number of threshold nodes ([Section 6.6](#66-threshold-nodes)) evaluate it. A metric keeps
no history; only its current sample counts.

A `MetricDefinition` declares a metric:

| Field | Type | Description |
| --- | --- | --- |
| `id` | string | Unique ID of the metric. |
| `owner` | string, optional | Owner of the definition ([Section 3.3](#33-owners)). |
| `label` | string, optional | Human-readable name of the metric. |
| `components` | list of strings, optional | IDs of associated components ([Section 7](#7-links-between-components-and-health)). |
| `unit` | unit | Unit of all samples and thresholds of the metric. |
| `maxAgeMs` | unsigned 32-bit integer, optional | Upper limit for the age of samples. MUST be positive. |
| `expectedTag` | string, optional | Tag that samples need to carry to be fresh. |

A `MetricSample` is evidence ([Section 6.3](#63-evidence)) with the following further fields:

| Field | Type | Description |
| --- | --- | --- |
| `id` | string | ID of the metric. |
| `value` | number | Measured value in the metric's unit. |

Values and range bounds are IEEE 754 binary64 numbers, encoded as JSON numbers. They MUST be
finite. The units are:

| Unit | Meaning |
| --- | --- |
| `celsius` | Degrees Celsius. |
| `bytes` | Bytes. |
| `seconds` | Seconds. |
| `ratio` | Dimensionless fraction, where 1 corresponds to 100 percent. |
| `count` | Number of items or events. |
| `bytes-per-second` | Bytes per second. |
| `hertz` | Cycles per second. |
| `volts` | Electric potential in volts. |
| `amperes` | Electric current in amperes. |
| `watts` | Power in watts. |
| `revolutions-per-minute` | Revolutions per minute. |

There MUST be at most one definition and at most one sample per metric ID. Samples without a
definition are ignored, but MUST still be valid.

> **Note:** Units describe what was measured. Tools choose how to display values, including
> precision, prefixes, and conversions. A `ratio` is not restricted to the range from 0 to 1.

### 6.6 Threshold Nodes

A threshold node derives its answers by comparing a metric sample with two ranges. Its rule has
the following fields:

| Field | Type | Description |
| --- | --- | --- |
| `metric` | string | ID of the metric. A definition with this ID must exist. |
| `healthy` | range | Range in which the desired criteria are met. |
| `usable` | range | Range in which the minimum criteria are met. |
| `maxAgeMs` | unsigned 32-bit integer, optional | Upper limit for the age of samples. MUST be positive. |

A range has an optional `lower` and an optional `upper` bound. Each bound has a `value` and a
boolean `inclusive`, which states whether the bound itself belongs to the range. A missing bound
leaves the range unbounded on that side. A value lies in a range if it is greater than the lower
bound, or equal to it and the bound is inclusive, and less than the upper bound, or equal to it and
the bound is inclusive.

A range MUST NOT be empty: if it has both bounds, the lower value MUST NOT exceed the upper value,
and if both values are equal, both bounds MUST be inclusive. The healthy range MUST be a subset of
the usable range.

The metric's sample is checked against the metric's definition and the rule: its owner against the
definition's `owner`, its tag against the definition's `expectedTag`, and its age against the
`maxAgeMs` of both. If the sample is fresh and the metric's unit is unknown to the evaluator, the
node's answers are `unknown`/`unknown`, with the reason `unsupported`. Otherwise, if the sample is
fresh, the node's answers are:

- `yes`/`yes` if the value lies in the healthy range,
- `yes`/`no` if it lies in the usable range but not in the healthy range, and
- `no`/`no` otherwise.

Writers MUST NOT publish status records for threshold nodes.

> **Example:** A threshold node with the healthy range `{"upper": {"value": 75, "inclusive": true}}`
> and the usable range `{"upper": {"value": 90, "inclusive": true}}` over a CPU temperature is
> healthy at 75 °C, usable but not healthy at 90 °C, and not usable above 90 °C.

> **Note:** Threshold nodes evaluate only the current sample. Smoothing a fluctuating measurement,
> for example, by averaging, is the responsibility of the service that publishes it.

### 6.7 Evaluation and Reports

A health evaluator evaluates a graph, the status records, the metric definitions, and the metric
samples in a health context. It produces a `HealthReport` with a list `nodes` that contains one
`HealthNodeResult` per node, in the order of the node definitions:

| Field | Type | Description |
| --- | --- | --- |
| `id` | string | ID of the node. |
| `owner` | string, optional | Owner of the node. |
| `usable` | answer | Whether the minimum criteria are met: `yes`, `no`, or `unknown`. |
| `healthy` | answer | Whether the desired criteria are met: `yes`, `no`, or `unknown`. |
| `inputs` | list of strings | IDs of the node's inputs, in the order of the rule; empty for other nodes. |
| `reason` | reason, optional | Why the node's evidence was not used ([Section 6.3](#63-evidence)). |
| `message` | string, optional | Message of the fresh evidence. |
| `details` | object, optional | Details of the fresh evidence. |

The reasons are `missing`, `otherBoot`, `ownerMismatch`, `tagMismatch`, `future`, and `expired`
for evidence that is not fresh, and `unsupported` for a rule type, state, or unit unknown to the
evaluator ([Section 3.4](#34-evolution)). Results of aggregate nodes have no reason; their inputs
explain them.

A graph is invalid if any of the following holds:

- two nodes have the same ID,
- an aggregate node refers to a node that does not exist, refers to a node more than once, marks an
  input as both required and optional, or has invalid counts
  ([Section 6.8](#68-aggregate-nodes)),
- the graph contains a cycle,
- a threshold node refers to a metric without a definition, or has an empty range or a healthy
  range that is not a subset of its usable range,
- two status records refer to the same node, or a status record refers to a threshold or aggregate
  node,
- two metric definitions or two metric samples have the same ID,
- a value or range bound is not finite, or
- a `maxAgeMs` is zero.

For an invalid graph, a health evaluator MUST report an error and MUST NOT produce a health
report.

> **Note:** An aggregate node that refers to a node of an app that may be absent makes the whole
> graph invalid while the app is absent. A node that combines nodes of several contributors
> therefore belongs to whoever knows which contributors are present.

### 6.8 Aggregate Nodes

An aggregate node derives its answers from other nodes, its _inputs_. Its rule has the following
fields:

| Field | Type | Description |
| --- | --- | --- |
| `inputs` | list of inputs | The node's inputs. |
| `required` | unsigned 32-bit integer, optional | Number of non-optional inputs that need to be usable. |
| `healthy` | unsigned 32-bit integer, optional | Number of non-optional inputs that need to be healthy. |

Each input has the following fields:

| Field | Type | Description |
| --- | --- | --- |
| `node` | string | ID of the input node. |
| `required` | boolean, optional | Whether the input is _mandatory_. Defaults to `false`. |
| `optional` | boolean, optional | Whether the input is _optional_. Defaults to `false`. |

An input MUST NOT be both mandatory and optional. Optional inputs only affect the desired criteria.
The counts apply to the non-optional inputs, which include the mandatory inputs. If `healthy` is
omitted, it equals the number of non-optional inputs, _N_. If `required` is omitted, it equals
`healthy`. After applying these defaults, the counts MUST satisfy 1 ≤ `required` ≤ `healthy` ≤ _N_.

The answers combine in three-valued logic. For a list of answers, their _conjunction_ is `no` if
any answer is `no`, otherwise `unknown` if any answer is `unknown`, and `yes` otherwise; the
conjunction of no answers is `yes`. For a count _k_ over a list of answers, the _count answer_ is
`yes` if at least _k_ answers are `yes`, `no` if fewer than _k_ answers are `yes` or `unknown`, and
`unknown` otherwise.

The answers of an aggregate node are:

- **Usable:** the conjunction of
  - the count answer for `required` over the usable answers of the non-optional inputs, and
  - the usable answers of the mandatory inputs.
- **Healthy:** the conjunction of
  - the node's usable answer,
  - the count answer for `healthy` over the healthy answers of the non-optional inputs,
  - the healthy answers of the mandatory inputs, and
  - the healthy answers of the optional inputs.

> **Example:** A telemetry app needs its database and its connection to a machine controller to
> collect data, and an upload to a cloud service to be fully healthy. While the upload fails, the
> app buffers data locally and remains usable:
>
> ```json
> {
>   "id": "telemetry",
>   "rule": {
>     "type": "aggregate",
>     "inputs": [
>       { "node": "telemetry.database" },
>       { "node": "telemetry.machine" },
>       { "node": "telemetry.upload", "optional": true }
>     ]
>   }
> }
> ```
>
> Without counts, both non-optional inputs must be usable for the app to be usable, and all inputs
> must be healthy for it to be healthy. If the upload's status record expires, the app's answers
> are `yes`/`unknown`. If the database is not usable, they are `no`/`no`.

> **Example:** With three interchangeable workers, `"required": 2` keeps a service usable while
> one worker is down, and the omitted `healthy` count requires all three workers for full health.
> If one worker is not usable and another is unknown, the service's usable answer is `unknown`,
> because whether two workers are usable depends on the unknown one; its healthy answer is `no`,
> because fewer than three workers can be healthy.

> **Note:** The summarized states of [Section 6.1](#61-health-answers) follow from the answers.
> For example, the usable answer `yes` with the healthy answer `no` or `unknown` is `degraded`.

## 7. Links Between Components and Health

The components part and the health part are independent. Compatibility checking does not consider
health, and health evaluation does not consider compatibility. The parts are linked in one
direction only: health nodes and metric definitions MAY name associated components in their
`components` field.

An association states that a node or metric concerns a component, for example, that a database
check concerns an app. It creates no dependency: a health evaluator MUST NOT consider whether
associated components exist, and associated components do not affect a node's answers. Tools MAY
use associations to present health per component, for example, to show the health nodes that
concern an app.

## 8. Discovery

This section defines how writers publish records as files on Linux and how readers discover them.

### 8.1 Directories

Each kind of record has standard directories:

| Records | Standard Directories |
| --- | --- |
| Component declarations | `/usr/lib/system-runtime/components`, `/etc/system-runtime/components`, `/run/system-runtime/components` |
| Health nodes | `/usr/lib/system-runtime/health/nodes`, `/etc/system-runtime/health/nodes`, `/run/system-runtime/health/nodes` |
| Status records | `/run/system-runtime/health/status` |
| Metric definitions | `/usr/lib/system-runtime/metrics/definitions`, `/etc/system-runtime/metrics/definitions`, `/run/system-runtime/metrics/definitions` |
| Metric samples | `/run/system-runtime/metrics/samples` |
| Directory configurations | `/usr/lib/system-runtime/directories`, `/etc/system-runtime/directories`, `/run/system-runtime/directories` |

> **Note:** `/usr/lib/system-runtime` holds records of the system image, `/etc/system-runtime`
> records specific to a device, and `/run/system-runtime` records generated at runtime. Evidence
> only has directories below `/run`, because it is only valid during the boot in which it was
> taken.

### 8.2 Files

Each record is a regular file with the extension `.json` that contains exactly one record of the
kind of its directory. The file name has no meaning; the record's ID determines its identity.

A writer MUST write a file so that readers never see a partially written file. It SHOULD write the
record to a temporary file with another extension in the same directory and then rename the
temporary file to the final name. There are no transactions across files.

Directories do not authenticate writers. Each status record and each metric sample MUST have
exactly one writer.

> **Note:** To let an unprivileged service publish its evidence without giving it access to the
> evidence of other services, give the service a directory of its own and add the directory with a
> directory configuration.

### 8.3 Reading

To read the records of one kind, a reader:

1. reads the directory configurations ([Section 8.4](#84-directory-configurations)) like any other
   records and appends their directories for that kind to the standard directories, in the order in
   which it read the configurations,
2. visits the directories in order and traverses each depth-first: it visits the entries of a
   directory in the order of the bytes of their names and descends into a subdirectory when it
   visits it,
3. reads every regular file with the extension `.json`, skipping files with other extensions and
   symlinks within a directory, and
4. reads each file at most once: a file whose canonical absolute path it has already read is
   skipped, even if another directory contains it. Different paths to the same file, such as hard
   links, count as different files.

A reader MUST skip directories that do not exist. A reader MUST report an error if a directory path
exists but is not a directory, if it is a broken symlink, or if a file cannot be read or does not
contain a valid record. Records in different directories do not override each other; records with
the same ID are subject to the rules of their part.

A directory MAY be a symlink. A reader MUST resolve each directory to its physical location before
traversing it, so that changing the symlink during traversal cannot mix files of two
directories.

> **Note:** Records can change at any time. Tools can reduce the risk of acting on outdated
> records by loading a fresh set before acting and retrying when they detect a concurrent change.

### 8.4 Directory Configurations

A `DirectoryConfig` adds directories for other contributors, such as apps that ship records in
their installation directories:

| Field | Type | Description |
| --- | --- | --- |
| `components` | list of paths, optional | Directories of component declarations. |
| `healthNodes` | list of paths, optional | Directories of health nodes. |
| `healthStatuses` | list of paths, optional | Directories of status records. |
| `metrics` | list of paths, optional | Directories of metric definitions. |
| `metricSamples` | list of paths, optional | Directories of metric samples. |

Paths MUST be absolute and MUST NOT contain `..` segments. A path segment consisting only of `*`
matches every directory at that level, in the order of the bytes of their names, but no symlink. Other wildcards, such as
`*.d` or `**`, are invalid. Readers MUST expand wildcards whenever they read records. Directory
configurations cannot add directories for directory configurations.

> **Example:** This configuration discovers the records of all installed apps of an app manager
> that keeps the active version of each app behind a `current` symlink:
>
> ```json
> {
>   "components": ["/var/lib/apps/*/current/components"],
>   "healthNodes": ["/var/lib/apps/*/current/health/nodes"]
> }
> ```

## 9. Transport

This section defines how health information is sent to another system, such as a
fleet-management server.

### 9.1 Snapshots

A `HealthSnapshot` contains everything needed to evaluate health on another system:

| Field | Type | Description |
| --- | --- | --- |
| `specVersion` | string | Version of this specification that the snapshot follows, such as `0.2`. |
| `clock` | clock sample | Correlation of the boot-time clock with wall-clock time. |
| `nodes` | list of health nodes | The health graph. |
| `statuses` | list of status records | The status records. |
| `metrics` | list of metric definitions, optional | Metric definitions. |
| `metricSamples` | list of metric samples, optional | Metric samples. |

A `RuntimeSnapshot` contains all records read from a system, in the fields `specVersion`,
`components`, `healthNodes`, `healthStatuses`, `metrics`, and `metricSamples`. The fields `metrics`
and `metricSamples` are optional.

Snapshots MUST contain the original records. In particular, transport MUST NOT change observation
times or validity durations. A health snapshot MUST contain the definition of every metric that a
threshold node refers to. Lists without records are empty. Optional lists without records MAY be
omitted.

The `specVersion` of a snapshot consists of the major and minor version of this specification, such
as `0.2`. A receiver MUST NOT interpret a snapshot unless it supports its version: from version 1.0
on, a receiver supports the versions with its own major version, applying the fallbacks of
[Section 3.4](#34-evolution) to newer minor versions; before version 1.0, a receiver only supports
its own minor version.

### 9.2 Clock Correlation

The receiver of a snapshot cannot interpret boot-time clock values on its own. A
`HealthClockSample` correlates them with wall-clock time:

| Field | Type | Description |
| --- | --- | --- |
| `context` | health context | Boot ID and boot-time clock value at sampling. |
| `timestamp` | timestamp | Wall-clock time at the same instant, in RFC 3339 format in UTC. |

The writer of a health snapshot MUST take the clock sample after collecting the records.
Timestamps MUST end in `Z` and MAY include fractional seconds.

Evaluating a snapshot with the context of its clock sample determines health at the time the
snapshot was taken. For evidence from the boot of the clock sample, its estimated wall-clock time
is the clock sample's `timestamp`, truncated to milliseconds, minus `nowMs − observedAtMs`
milliseconds. The estimate has millisecond precision. Evidence from another boot, evidence with an
observation time after the clock sample, and evidence whose estimate lies outside the years 0001 to
9999 have no estimated wall-clock time.

> **Example:** With the clock sample `{"context": {"bootId": "…", "nowMs": 45000}, "timestamp":
> "2026-10-14T12:00:00Z"}`, a status record observed at 42000 ms was observed at about
> `2026-10-14T11:59:57Z`. The estimate is only as accurate as the wall clock at the time of
> sampling.
