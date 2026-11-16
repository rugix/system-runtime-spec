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
2. [Conformance](#2-conformance)
3. [Records](#3-records)
4. [Versions and Version Requirements](#4-versions-and-version-requirements)
5. [Components](#5-components)
6. [Health](#6-health)
7. [Links Between Components and Health](#7-links-between-components-and-health)
8. [Discovery](#8-discovery)
9. [Transport](#9-transport)
10. [Limitations](#10-limitations)
11. [Security Considerations](#11-security-considerations)

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

The specification distinguishes five roles:

- A **writer** publishes records or snapshots, such as a system image that ships its component
  declaration or a service that publishes the results of its checks.
- A **reader** discovers and parses records ([Section 8](#8-discovery)) or receives snapshots
  ([Section 9](#9-transport)).
- A **compatibility checker** evaluates component declarations
  ([Section 5](#5-components)).
- A **health evaluator** evaluates health graphs ([Section 6](#6-health)).
- A **consumer** acts on reports, such as an update controller that decides whether to commit an
  update.

An implementation can take several roles. For example, an update controller reads records, checks
the compatibility of an update, writes the records of the update, and decides whether to commit it.
Records come from _contributors_, such as a system image or an app, whose writers publish them.

The sections that matter most for each role are:

| Role | Sections |
| --- | --- |
| Writer | [3](#3-records), [5.1–5.3](#51-component-declarations) for component declarations, [6.1–6.7](#61-health-answers) for health records, and [8.1–8.2](#81-directories) |
| Reader | [3](#3-records), [8](#8-discovery), and [9](#9-transport) |
| Compatibility checker | [3](#3-records), [4](#4-versions-and-version-requirements), and [5](#5-components) |
| Health evaluator | [3](#3-records) and [6](#6-health) |
| Consumer | [5.5](#55-compatibility-reports), [6.1](#61-health-answers), [6.8](#68-evaluation-and-reports), and [10](#10-limitations) |

An implementation conforms to the specification for a role if it satisfies all requirements that
apply to that role. The [conformance suite](tests/fixtures/) in this repository
contains test cases for version matching, compatibility checking, candidate compositions, health
evaluation, snapshots, clock correlation, and discovery. Passing it is evidence of conformance, but
does not replace the requirements of the specification. If a test case contradicts the
specification, the specification applies, and the test case is wrong.

## 3. Records

Records are small JSON documents that the parts of a system publish about themselves. This section
defines what all records have in common: their encoding, how problems with them are reported, their
IDs and owners, data for other tools, and how the format evolves.

### 3.1 Encoding

Each record is a JSON object of one of the record types of the schemas, such as
`ComponentDeclaration` or `HealthStatus`. Field names use camelCase, so the schema field
`max_age_ms` is the JSON field `maxAgeMs`, which this document uses. Enumeration values use
PascalCase, and absent optional fields are omitted. Fields of the type `u64`, such as boot-time
clock values, hold unsigned 64-bit integers; values beyond 2⁵³ − 1 are decimal strings, so that
JavaScript implementations can process them without losing precision.

A record is _valid_ if it has the structure of its schema and its fields satisfy the conditions
that this document and the schemas state for them, such as that a version is valid or that a
`maxAgeMs` is positive. Unknown fields, unknown enumeration values
([Section 3.6](#36-evolution)), and unit codes ([Section 6.5](#65-metrics)) do not make a record
invalid. Conditions that concern several records, such as unique IDs, do not affect validity; the
parts define their consequences. Writers MUST NOT publish invalid records.

### 3.2 Diagnostics

Records come from many contributors, and one faulty record should not prevent the evaluation of the
others. Readers, compatibility checkers, and health evaluators MUST skip invalid records and records
that cannot be read, and MUST report a _diagnostic_ for each:

```sidex
/// A record that was skipped or used despite a problem.
record Diagnostic {
    /// Whether the results may not reflect the records concerned.
    severity: Severity,
    /// Kind of records concerned.
    kind: RecordKind,
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

/// Kind of records, named after the record types.
#[json(tagged = externally)]
variant RecordKind {
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

The kind of a diagnostic is the kind of the records it concerns, also for a directory of such
records and for diagnostics added during evaluation. The kind `DirectoryConfig` is reserved for
directory configurations themselves. The components part and the health part define how
diagnostics affect their results ([Sections 5.4](#54-consistency) and
[6.8](#68-evaluation-and-reports)). The order of diagnostics is not specified.

### 3.3 Identifiers

Records are identified by IDs. IDs, owners ([Section 3.4](#34-owners)), and the IDs of
capabilities and claims ([Section 5](#5-components)) are non-empty strings of the printable ASCII
characters `!` (U+0021) to `~` (U+007E). They are case-sensitive, compared by exact equality, and
sorted by their bytes.

Each kind of record has its own namespace: a component, a health node, and a metric MAY share an
ID without referring to each other. Within a namespace, the records of all contributors share the
IDs. The IDs of components, health nodes, and metrics SHOULD therefore start with a prefix that the
contributor controls, followed by a dot, such as `telemetry.database`.

### 3.4 Owners

Component declarations, health nodes, status records, metric definitions, and metric samples have
an optional `owner`. The owner names the contributor that supplies the record, such as a system
image or an app, and that replaces or removes the record as a whole.

Owners do not scope IDs: records of different owners need distinct IDs within their namespace, and
the parts define what happens if they share one. Records without an owner are valid.

An update of a contributor replaces all component declarations, health nodes, and metric definitions
of its owner, including overrides in `/etc` ([Section 8.3](#83-reading)). A writer that performs an
update MUST ensure that, once the updated records take effect, no such records of the replaced owner
from before the update remain in the directories that readers read. A writer that rolls back an
update SHOULD restore the records that the update removed. [Section 5.6](#56-candidate-compositions)
defines how compatibility checkers predict the result of an update, and [Section 6.3](#63-evidence)
how owners constrain evidence.

> **Note:** A system image might use the owner `rugix.system`, and an app named `telemetry` the
> owner `rugix.apps.telemetry`. Owner names are conventions of the tools that use them; readers do
> not derive owners from file paths.

### 3.5 Annotations

Component declarations, health nodes, metric definitions, and evidence have an optional
`annotations` field, an object for data that this specification does not define, such as
information for a particular tool. Member names SHOULD start with a reverse domain name that the
writer controls, such as `com.example.priority`. Annotations do not affect compatibility checking
or health evaluation, and results do not carry them.

Evidence ([Section 6.3](#63-evidence)) also has an optional `details` field, an object that
explains one observation to people, such as the output of a check, in any structure. The result
of a node carries the details of its fresh evidence. Writers SHOULD put data for tools into
annotations and data for display into details.

Writers MUST put data that this specification does not define into annotations or details, and
MUST NOT add other fields to records.

### 3.6 Evolution

The specification is versioned with `MAJOR.MINOR` version numbers. A minor version only makes
changes that readers of earlier minor versions of the same major version can safely ignore:

- adding optional fields whose absence keeps the meaning of a record unchanged, and
- adding values to the enumerations that this section lists, with the fallback defined here.

Every record has an optional field `specVersion`, which holds the version of this specification that
the record follows, such as `1.0`. A record without it follows version 1.0. Writers SHOULD set it to
the lowest version whose features the record uses. Specification versions, here and in snapshots
([Section 9.1](#91-snapshots)), have the following form, where both numbers fit into an unsigned
64-bit integer and compare numerically:

```text
spec-version = number "." number
number       = "0" | nonzero { digit }
nonzero      = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
```

A record whose `specVersion` does not have this form, or has a major version that the reader does
not support, is invalid. A component declaration with a higher minor version than the reader
supports is invalid as well, because a matching feature that the reader does not know could make
it match too much. For other records with a higher minor version, the reader uses them, applying
the fallbacks of this section, and reports a diagnostic with the severity `Warning`.

The schemas describe what writers produce. Implementations MUST ignore fields they do not know,
and MUST treat values they do not know as follows:

| Enumeration | Role | Fallback |
| --- | --- | --- |
| Health rule types ([Section 6.2](#62-health-graphs)) | Health evaluator | The node's result is unknown, with the reason `Unsupported`. |
| Status states ([Section 6.4](#64-observed-nodes-and-status-records)) | Health evaluator | The node's result is unknown, with the reason `Unsupported`. |
| Input modes ([Section 6.7](#67-aggregate-nodes)) | Health evaluator | The aggregate node's result is unknown, with the reason `Unsupported`. |
| Compatibility problems ([Section 5.5](#55-compatibility-reports)) | Consumer | The problem is reported without further detail. |
| Reasons ([Section 6.8](#68-evaluation-and-reports)) | Consumer | The result is treated as having no reason. |
| Diagnostic kinds ([Section 3.2](#32-diagnostics)) | Reader of a snapshot, consumer | The diagnostic is treated as of the kind `DirectoryConfig`. |
| Diagnostic severities ([Section 3.2](#32-diagnostics)) | Reader of a snapshot, consumer | The diagnostic is treated as an error. |

> **Note:** The fallbacks are conservative. For example, a node that attaches to an aggregate node
> with an input mode added in a later minor version makes that aggregate node unknown for older
> health evaluators.

A change that readers cannot safely ignore requires a new major version. If records of a new major
version appeared next to records of an earlier one, readers of the earlier major version would
report them as invalid, which would make compositions inconsistent and block health decisions. A new
major version therefore uses new standard directories ([Section 8.1](#81-directories)).

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
components and prerelease identifiers that consist only of digits are _numeric_ and MUST fit into an
unsigned 64-bit integer. Leading zeros are allowed. Whitespace at the start and end of a version is
ignored. In this document, _whitespace_ means the characters with the Unicode property
`White_Space`.

> **Note:** The grammar covers, for example, `1.4.2`, `2.0.0-rc.1`, `1.4.2+build.7`, `2026.06.25`,
> `42`, and `20260625071935`. Versions of other formats must be converted, which can lose
> information. For example, the Debian version `2.27~rc1` could become `2.27-rc.1`, where the
> separate numeric identifier keeps `rc.2` before `rc.10`. Debian epochs and revisions have no
> counterpart and need a convention of the converting tool.

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
optionally followed by the wildcard `.*`:

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
> contain its changes. Package managers such as Cargo and npm exclude prereleases unless a
> requirement names one, because they choose which versions to install. Requirements in this
> specification check versions that are already present, and a release candidate is meant to work
> like the release of its line.

> **Note:** The operators `==` and `!=` compare precedence, and the other operators compare lines.
> So `<=3` matches `3.5`, but `==3` does not. The requirement `3.*` matches the line `3`. Build
> metadata does not affect precedence, so `==3+build.7` matches every build of `3`.

## 5. Components

A _component_ is one part of a system, such as the system image, an app, a runtime, or a hardware
module. Together, the components of a system form its _composition_. Components declare what they
provide and require, and a compatibility checker determines whether the composition is
_consistent_.

### 5.1 Component Declarations

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

The `version` MUST be valid ([Section 4.1](#41-versions)).

### 5.2 Capabilities

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
  valid ([Section 4.1](#41-versions)).
- A capability with a `value` states an exact property as a string.

A capability MUST NOT have both a `version` and a `value`.

Every component also provides an _implicit capability_ with its own `id` and, if it declares one,
its `version`.

> **Example:** `{"id": "docker.compose", "version": "2.27.0"}` states which version of Docker
> Compose is present, and `{"id": "host.arch", "value": "aarch64"}` states the CPU architecture.

### 5.3 Selectors and Claims

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

The `version` of a selector holds a version requirement ([Section 4.3](#43-version-requirements)),
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

> **Note:** Each requirement is met on its own, possibly by different components. A component
> that needs two properties of the same provider requires one capability that states both. Other
> constraints follow from conflicts and shared capabilities. A component that works with a library
> only from version 2 on, if the library is present at all, declares the conflict
> `{"id": "libfoo", "version": "<2"}`. Components that are alternatives to each other, such as two
> database engines, provide a common capability, such as `database.sql`, that other components
> require.

> **Note:** Capabilities and claims only match if all components name them the same way, so their
> IDs need shared conventions, such as `network.tcp.8080` for a TCP port.

### 5.4 Consistency

A compatibility checker evaluates a composition, which is a list of component declarations, together
with the diagnostics about component declarations and directory configurations
([Section 3.2](#32-diagnostics)). It skips declarations that are not valid records, for example,
because they contain an invalid version or version requirement, or a capability or selector with
both a version and a value, and adds a diagnostic with the severity `Error` for each. It considers
every explicit and implicit capability of every remaining component. The composition is _consistent_
if and only if:

- there are no diagnostics with the severity `Error`,
- no two components have the same ID,
- no two components claim the same resource,
- every requirement of every component matches at least one capability, and
- no conflict of any component matches any capability, including the capabilities of the
  component declaring the conflict.

Several components MAY provide the same capability. Consistency covers only the declared
constraints ([Section 10](#10-limitations)).

### 5.5 Compatibility Reports

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

Problems contain the selectors and capabilities as declared, with the fields that the schema
defines. Two problems are _equal_ if they have the same kind and equal fields. Two diagnostics are
_equal_ if they have equal fields, apart from their messages.

### 5.6 Candidate Compositions

A _candidate composition_ is the composition that would result from an update. An update replaces
all declarations of one or more owners ([Section 3.4](#34-owners)): it names these owners and
supplies new declarations, each of which MUST have one of them. A compatibility checker skips a new
declaration that does not have a replaced owner and adds a diagnostic with the severity `Error` for
it. An update that installs a new contributor replaces an owner that has no declarations yet, and
an update that removes a contributor supplies no declarations for its owner.

The candidate composition is built from the current composition:

1. Remove the declarations with a replaced owner.
2. Append the new declarations, in the order that the update supplies them.

Its diagnostics are the current diagnostics and those about the new declarations. Checking the
candidate composition finds broken requirements before the update takes place, because the
remaining declarations keep their requirements. A snapshot ([Section 9.1](#91-snapshots)) contains
the current composition, so a fleet-management server can check an update before sending it.

> **Note:** Declarations without an owner are never removed. This suits facts about the device
> itself, such as the revision of its board. A declaration that is meant to be replaced needs an
> owner; otherwise, its replacement duplicates its ID.

> **Note:** If the current composition is already inconsistent, every candidate composition that
> keeps the existing problems is inconsistent too, including an update that fixes another problem. A
> consumer can accept a candidate composition whose problems and error diagnostics each equal one of
> the current composition, provided that the update adds no diagnostics about its own declarations,
> so that existing problems do not block updates that do not make them worse. Current diagnostics
> about files that the update replaces stay in the candidate composition and therefore count as
> existing.

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
 The answers are related: a node that is healthy
is usable, and a node that is not usable is not healthy. Therefore, exactly six combinations are
possible:

| Usable | Healthy | Meaning |
| --- | --- | --- |
| `Yes` | `Yes` | The desired criteria are met. |
| `Yes` | `No` | The node is usable, but the desired criteria are not met. |
| `Yes` | `Unknown` | The node is usable, but whether the desired criteria are met is unknown. |
| `Unknown` | `No` | The desired criteria are not met, and whether the node is usable is unknown. |
| `Unknown` | `Unknown` | Whether the node is usable is unknown. |
| `No` | `No` | The node is not usable. |

For display and for publishing, the combinations are summarized as four _health states_:

| State | Answers |
| --- | --- |
| `Healthy` | Healthy `Yes`. |
| `Degraded` | Usable `Yes` and healthy `No` or `Unknown`. |
| `Unhealthy` | Usable `No`. |
| `Unknown` | Usable `Unknown`. |

A status record ([Section 6.4](#64-observed-nodes-and-status-records)) reports one of these states,
which corresponds to the answers `Yes`/`Yes`, `Yes`/`No`, `No`/`No`, or `Unknown`/`Unknown`.

> **Note:** A consumer that makes a decision, such as an update controller deciding whether to
> commit an update, chooses the node and the answer it requires. Requiring the healthy answer `Yes`
> demands the desired criteria, including those of optional inputs
> ([Section 6.7](#67-aggregate-nodes)). Requiring the usable answer `Yes` demands the minimum
> criteria. The answer `Unknown` satisfies neither.

### 6.2 Health Graphs

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

[Sections 6.4](#64-observed-nodes-and-status-records), [6.6](#66-threshold-nodes), and
[6.7](#67-aggregate-nodes) define the rules, and [Section 7](#7-links-between-components-and-health)
the associated components. A node whose rule type is unknown
to the evaluator has the answers `Unknown`/`Unknown`, with the reason `Unsupported`.

The inputs of aggregate nodes, including attached inputs, form directed edges from each aggregate
node to its inputs. The graph SHOULD be acyclic; nodes on a cycle are not evaluated
([Section 6.8](#68-evaluation-and-reports)).

Nodes that are no input of any other node are _roots_. Tools can show the roots of a graph as the
overall health of a device. Which aggregate nodes contributors attach to is a convention of the
platform that combines them, like the names of owners ([Section 3.4](#34-owners)).

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
> node whose inputs attach themselves cannot tell which contributors are absent; a minimum count
> ([Section 6.7](#67-aggregate-nodes)) can detect too few inputs, but does not establish which
> contributors are present.

### 6.3 Evidence

Status records and metric samples are _evidence_: results of checks and measurements taken on the
running system. Evidence is only valid for a limited time, so that a check that stops running
cannot keep reporting an old result. Times are measured on the _boot-time clock_ of the observed
system, which starts at zero when the system boots and counts suspended time. Unlike a wall clock,
it does not change when the system adjusts its time.

Status records ([Section 6.4](#64-observed-nodes-and-status-records)) and metric samples
([Section 6.5](#65-metrics)) share the fields of evidence: `bootId`, `observedAtMs`, `maxAgeMs`,
`owner`, `tag`, `message`, `details`, and `annotations`.

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
used, and the node's answers are `Unknown`/`Unknown` with that reason. Only fresh evidence is
interpreted: if it is fresh, the node's result carries its `message` and `details`, and the rules of
the node's type determine the answers.

> **Example:** A status record observed at 42000 ms with a `maxAgeMs` of 15000 is fresh until
> just before 57000 ms. If its node declares a `maxAgeMs` of 10000, it is fresh until just before
> 52000 ms.

The boot ID and the age do not show that evidence comes from the current instance of a service.
If an update replaces an app during a boot, the evidence of the previous instance stays fresh
until it expires. A writer whose evidence depends on the instance or configuration that it checks
SHOULD set `tag` to identify it, such as the version of an app, and the node and metric
definitions of the same contributor SHOULD declare the matching `expectedTag`.

### 6.4 Observed Nodes and Status Records

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
    /// Further information about the observation, for display, in any structure.
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
in [Section 6.1](#61-health-answers). If the state is unknown to the evaluator, the node's answers
are `Unknown`/`Unknown`, with the reason `Unsupported`.

> **Note:** A check reports `Degraded` when it is usable but not fully healthy, for example, a
> disk check that sees little free space. It reports `Unknown` when it cannot reach a conclusion,
> for example, because a dependency of the check itself is unavailable.

> **Note:** A status record cannot express the answers `Yes`/`Unknown` and `Unknown`/`No`, so it
> cannot relay every result of an evaluation. A system that passes on the health of other
> systems, such as a gateway for attached devices, passes on a separate snapshot for each system
> ([Section 9](#9-transport)), so that their answers are evaluated from the original records in
> their own health contexts. Combining the health of several systems is outside the scope of this
> specification.

### 6.5 Metrics

A _metric_ is a named numeric measurement, such as a CPU temperature or the free space on a
partition. Metrics separate measurements from health policies: a service publishes a measurement,
and any number of threshold nodes ([Section 6.6](#66-threshold-nodes)) evaluate it. A metric keeps
no history; only its current sample counts.

A metric definition declares a metric, and a metric sample is evidence
([Section 6.3](#63-evidence)) of its current value:

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
    /// Further information about the observation, for display, in any structure.
    details?: [string: json],
    /// Further data that the specification does not define, with namespaced member names.
    annotations?: [string: json],
}
```

A metric ID names one measurement in one unit, because threshold nodes of other contributors
compare its samples with fixed numbers. Writers MUST NOT change the meaning or the unit of a
metric; a different measurement or unit needs a new ID. Writers MUST NOT publish two definitions
or two samples for the same metric. If they do, the answers of threshold nodes over the metric
are unknown ([Section 6.6](#66-threshold-nodes)). Samples without a definition are ignored.

Units are codes of the [Unified Code for Units of Measure][ucum] (UCUM) in its case-sensitive
form, which OpenTelemetry also uses. UCUM can express any unit, including derived units such as
`By/s` and terms in curly braces such as `{packet}/s`, which name what is counted. Writers
MUST use a valid UCUM code and SHOULD use a base unit without a prefix, such as `s` and `By`, so
that thresholds of different contributors compare without conversion. Dimensionless values use
`1`. Common measurements use the following codes:

| Measurement | Code | Unit |
| --- | --- | --- |
| Temperature | `Cel` | Degrees Celsius. |
| Information | `By` | Bytes. |
| Duration | `s` | Seconds. |
| Fraction | `1` | Dimensionless; 1 corresponds to 100 percent. |
| Number of items or events | `{item}`, such as `{process}` | Items, named by the annotation. |
| Data rate | `By/s` | Bytes per second. |
| Frequency | `Hz` | Hertz. |
| Rotational speed | `{rev}/min` | Revolutions per minute. |
| Electric potential | `V` | Volts. |
| Electric current | `A` | Amperes. |
| Power | `W` | Watts. |
| Energy | `J` | Joules. |
| Pressure | `Pa` | Pascals. |
| Length | `m` | Meters. |
| Signal power level | `dB[mW]` | Decibels relative to one milliwatt (dBm). |

Units describe what was measured. Health evaluation compares units only as strings
([Section 6.6](#66-threshold-nodes)) and never converts values. Readers MUST NOT check unit codes
and MUST NOT reject a metric because of its unit code.

> **Note:** Tools choose how to display values, including precision, prefixes, and conversions,
> and display codes they do not know as written. A fraction with the unit `1` is not restricted to
> the range from 0 to 1.

[ucum]: https://ucum.org/ucum

### 6.6 Threshold Nodes

A threshold node derives its answers by comparing a metric sample with two ranges:

```sidex
/// Ranges that a metric sample is compared with.
record ThresholdRule {
    /// ID of the metric.
    metric: MetricId,
    /// Unit of the ranges, as a UCUM code; must equal the metric's unit.
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

### 6.7 Aggregate Nodes

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

If an input or attachment has a mode unknown to the evaluator, the aggregate node's answers are
`Unknown`/`Unknown`, with the reason `Unsupported`. If a node is an input more than once, listed or
attached, it counts as one input, at the position of its first occurrence. Its mode is the mode of
the first input of the rule that refers to it or, if the rule does not list it, the mode of its
first attachment, so that the aggregate node's own rule decides how its inputs count. An input of
the rule that refers to no node has the answers `Unknown`/`Unknown`, and the evaluator adds a
diagnostic with the severity `Warning` for it; the input appears in the node's result, but has no
result of its own.

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
> Without counts, both counted inputs must be usable for the app to be usable, and all inputs must
> be healthy for it to be healthy. If the upload's status record expires, the app's answers are
> `Yes`/`Unknown`. If the database is not usable, they are `No`/`No`. While the app is installed,
> `rugix.apps` is only healthy if the app is; when the app is removed, its attachment is removed
> with it.

> **Example:** With three interchangeable workers, `"minUsable": 2` keeps a service usable while
> one worker is down, and the omitted `minHealthy` requires all three workers for full health. If
> one worker is not usable and another is unknown, the service's usable answer is `Unknown`,
> because whether two workers are usable depends on the unknown one; its healthy answer is `No`,
> because fewer than three workers can be healthy.

> **Note:** The summarized states of [Section 6.1](#61-health-answers) follow from the answers.
> For example, the usable answer `Yes` with the healthy answer `No` or `Unknown` is `Degraded`.

### 6.8 Evaluation and Reports

A health evaluator evaluates the health nodes, status records, metric definitions, and metric
samples in a health context, together with the diagnostics about these records and about directory
configurations ([Section 3.2](#32-diagnostics)). It proceeds as follows:

1. It skips invalid records, for example, nodes with an empty range or evidence with a `maxAgeMs`
   of zero, and adds a diagnostic with the severity `Error` for each.
2. If several nodes have the same ID, it evaluates none of them. The ID has one result, at the
   position of the first of these nodes, with the answers `Unknown`/`Unknown`, the reason
   `Conflict`, no owner, and no inputs. These nodes do not attach to other nodes, and attachments
   to their ID are ignored. The evaluator adds a diagnostic with the severity `Error` for the ID.
3. It adds the attached inputs of the remaining aggregate nodes
   ([Section 6.7](#67-aggregate-nodes)).
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
  that is not fresh ([Section 6.3](#63-evidence)), and `Missing` also for a metric without a
  definition,
- `Conflict` for records that contradict each other, such as two nodes with the same ID,
- `Cycle` for a node on a cycle, and
- `Unsupported` for a rule type, state, or input mode unknown to the evaluator
  ([Section 3.6](#36-evolution)).

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
nodes that use them unknown, unless a writer published another record with the same ID, which
[Sections 6.4](#64-observed-nodes-and-status-records) and [6.5](#65-metrics) forbid. A consumer can
also be configured to require a report without any error diagnostics.

## 7. Links Between Components and Health

The components part and the health part are independent. Compatibility checking does not consider
health, and health evaluation does not consider compatibility. The parts are linked in one
direction only: health nodes and metric definitions MAY name associated components in their
`components` field.

An association states that a node or metric concerns a component, for example, that a database
check concerns an app. It creates no dependency: a health evaluator MUST NOT consider whether
associated components exist, and associated components do not affect a node's answers. Consumers
MAY use associations to present health per component, for example, to show the health nodes that
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

The directories form three _levels_. From highest to lowest, they are the directories below `/etc`,
the directories below `/run`, and the directories below `/usr/lib` together with the directories
added by directory configurations ([Section 8.4](#84-directory-configurations)).

### 8.2 Files

Each record is a regular file with the extension `.json` that contains exactly one record of the
kind of its directory. The file name has no meaning; the record's ID determines its identity.

A writer MUST write a file so that readers never see a partially written file. It SHOULD write the
record to a temporary file in the same directory, whose name starts with `.` or has another
extension, and then rename the temporary file to the final name. There are no transactions across
files.

Directories do not authenticate writers ([Section 11](#11-security-considerations)).

> **Note:** To let an unprivileged service publish its evidence without giving it access to the
> files of other services, give the service a directory of its own and add the directory with a
> directory configuration. This separates files, but does not restrict the IDs that the service can
> publish.

### 8.3 Reading

To read the records of one kind, a reader first reads the directory configurations
([Section 8.4](#84-directory-configurations)) like any other records. It then visits the standard
directories in the order `/usr/lib`, `/etc`, `/run`, followed by the directories that the
configurations add for that kind, in the order in which it read the configurations. It traverses
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
([Section 3.2](#32-diagnostics)). A path whose existence cannot be determined is reported in the
same way.

Each record has the level of the directory through which the reader read it. A file that the reader
reaches through directories of several levels has the highest of these levels. Component
declarations, health nodes, and metric definitions override records of the same kind with the same
ID and the same owner from lower levels: of these records, a reader keeps only those from the
highest level, at their own positions. Two records have the same owner if both have none or both
have equal owners. Records with the same ID and different owners do not override each other; like
evidence with the same ID, they are subject to the rules of their part. Invalid records do not
override other records. Directory configurations have no IDs and cannot be overridden.

> **Note:** An override carries the owner of the record it overrides, so the next update of that
> owner removes it ([Section 3.4](#34-owners)) and it cannot outlive the record it changes. Whoever
> manages a device's overrides, such as a fleet-management tool, applies them again after updates.
> To change a threshold, an operator copies its node to `/etc/system-runtime/health/nodes` and
> changes the ranges. To disable a check, an operator overrides its node with an aggregate node
> without inputs, such as `{"id": "telemetry.upload", "owner": "rugix.apps.telemetry", "rule":
> {"type": "Aggregate"}}`, which is healthy as long as no other node attaches to it.

A directory MAY be a symlink. A reader MUST resolve each directory to its physical location before
traversing it, so that changing the symlink during traversal cannot mix files of two
directories.

> **Note:** Records can change at any time. Tools can reduce the risk of acting on outdated
> records by loading a fresh set before acting and retrying when they detect a concurrent change.

### 8.4 Directory Configurations

A directory configuration adds directories for other contributors, such as apps that ship records
in their installation directories:

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

## 9. Transport

This section defines how records are sent to another system, such as a fleet-management server,
so that it can check compatibility or evaluate health. Snapshots and reports use the encoding of
[Section 3.1](#31-encoding). A snapshot does not identify the system it describes; the transport
does.

### 9.1 Snapshots

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
after overrides ([Section 8.3](#83-reading)), and MUST include either all lists of the health part
or none. It MUST include the diagnostics of both severities that its reader reported about the
included kinds of records and about directory configurations, and omits `diagnostics` if there are
none.

A snapshot MUST contain the original records. A system that relays a snapshot is a writer of it
and MUST NOT change it, in particular not the observation times or validity durations.

The `specVersion` of a snapshot is the version of this specification that the snapshot's own fields
follow, such as `1.0` ([Section 3.6](#36-evolution)). The records in it keep their own versions. A
reader MUST NOT interpret a snapshot unless it supports its version. A reader supports the versions
with its own major version, and applies the fallbacks of [Section 3.6](#36-evolution) to newer minor
versions.

### 9.2 Clock Correlation

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
milliseconds. The estimate has millisecond precision. Evidence from another boot, evidence with an
observation time after the clock sample, and evidence whose estimate lies outside the years 0001 to
9999 have no estimated wall-clock time.

> **Example:** With the clock sample `{"context": {"bootId": "…", "nowMs": 45000}, "timestamp":
> "2026-10-14T12:00:00Z"}`, a status record observed at 42000 ms was observed at about
> `2026-10-14T11:59:57Z`. The estimate is only as accurate as the wall clock at the time of
> sampling.

## 10. Limitations

This section is informative. It states what the results of compatibility checking and health
evaluation do not establish.

- A consistent composition meets the declared constraints. Whether its components work together
  also depends on constraints that nobody declared.
- Health evaluation interprets evidence: the results of checks and measurements at the time they
  were taken. A healthy result does not show that a node stayed healthy between observations. How
  long a node needs to stay healthy before a consumer acts, for example, before it commits an
  update, is the consumer's decision.
- Which node a consumer bases a decision on, and which answer it requires, is its decision
  ([Section 6.1](#61-health-answers)).
- Records can change while a reader reads them, and there are no transactions across files
  ([Section 8.2](#82-files)). A snapshot holds the records as they were collected, which need not
  correspond to a single instant.
- Evaluating a snapshot establishes health at its clock sample, not at the time it is received or
  displayed. A consumer that shows the current health of a device needs its own policy for old
  snapshots and lost contact.
- A record that could not be read is missing from the results. Diagnostics report it
  ([Section 3.2](#32-diagnostics)).
- Owners and tags detect mistakes, such as outdated evidence. They do not authenticate writers
  ([Section 11](#11-security-considerations)).

## 11. Security Considerations

Records decide whether updates are installed, committed, or rolled back. A writer that publishes
false records can therefore block updates, cause rollbacks, or get an incompatible or faulty update
committed.

**Trust.** Readers trust every writer that can create files in the directories they read, for every
record such a writer publishes, including its ID and owner. Such a writer can attach nodes to any
aggregate node and cause conflicts with the records of others, and a writer in a higher level can
override the records of lower levels. Writers in `/run/system-runtime` can therefore override the
records of the system image. Owners and tags detect mistakes, but anyone who can write a record can
set them. A directory of its own separates the files of a service, but the service can still publish
records with any ID. Standard directories and added directories should therefore be writable only
by privileged users and by the services that publish into them. Restricting which IDs a writer may
publish needs a mechanism outside this specification, such as a collector that checks the records
of each source.

**Resource limits.** Readers SHOULD limit the size of files, the number of files, the depth of
directories, and the nesting of JSON values, and SHOULD skip what exceeds a limit and report a
diagnostic.

**Concurrent changes.** A writer can replace a file or a directory with a symlink after a reader
has checked it. Readers SHOULD traverse directories relative to open file descriptors and open
entries without following symlinks, for example, with `openat` and `O_NOFOLLOW` on Linux.

**Transport.** Snapshots reveal information about a device, such as its software and the messages
and details of its checks. Writers SHOULD NOT put secrets into labels, messages, details, or
annotations. Systems that send snapshots SHOULD protect their integrity and confidentiality and
authenticate the sending device, for example, with TLS.

**Display.** Labels, messages, and details are arbitrary text from writers. Consumers SHOULD treat
them as untrusted when they display them, for example, by escaping them in HTML.
