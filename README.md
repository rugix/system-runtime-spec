# System Runtime Specification

This repository contains the *System Runtime Specification*, developed by the
Rugix Project. It defines a common format for describing a system's components,
their compatibility requirements, their metrics, and their health.

A system image and its applications may be maintained and updated independently.
An image update can remove a runtime that an installed application still needs.
Even when a device boots and its services start, an application may be unusable
because it cannot reach its database. Accepting such an update can leave the
device unable to perform its intended function.

A shared format and common evaluation rules let several tools reuse the
component and health information each contributor publishes. Update controllers
can check a proposed update against installed applications before activation,
then use fresh health evidence to decide whether to accept the activated system.
Diagnostic and monitoring tools can use the same information to show affected
functions, the checks behind their health, and when those checks last ran.

## Concepts

### Composition and Compatibility

A system can consist of a system image, independently updated applications,
and hardware modules. These components form its composition. Components declare
capabilities they provide and requirements the composition must satisfy. For example,
an application might require a runtime version provided by the system image.

Components can also declare conflicts with capabilities they cannot coexist
with and claim exclusive use of resources, such as a TCP port. Compatibility
checking matches these constraints against the candidate composition and detects
competing claims.

### System Health

Health uses runtime evidence to describe whether a system's functions behave
as expected. A health graph connects checks to the functions they support.
Checks can receive health states published by services or derive health from
numeric measurements. Aggregate nodes combine results to describe larger
functions. An
application's health might depend on a responsive process and an available
database, with the same database check contributing to several applications.
These health dependencies are declared separately from compatibility
requirements.

Health evaluation distinguishes minimum criteria for remaining usable from
desired criteria for full health. A healthy result confirms the desired
criteria; degraded confirms the minimum while desired criteria remain
unconfirmed or are unmet. Unhealthy means minimum criteria are unmet. Unknown
means the evidence cannot settle whether minimum criteria are met.

Aggregation can express redundancy. A service with three replicas might need
two functioning replicas to remain usable, while full health requires all three
to be healthy. If one fails or stops reporting and two remain healthy, the
service is degraded. Operators can see that redundancy needs attention while
the service remains available.

Node definitions are separate from statuses, so a system image can declare a
check and a runtime service can publish its status. If the service stops
publishing, the check remains defined and becomes unknown when its status
expires.

Status ages use time within one boot, so clock corrections cannot make old
evidence appear fresh. Statuses from earlier boots are rejected. A check can
also require evidence tied to an application instance or configuration.
Updating that requirement for a replacement keeps a recent status from the old
instance from confirming the replacement's health.

A graph can expose several top-level results, called roots. Separate roots can
describe availability and update readiness using the same underlying checks.

### Metrics

Metrics expose numeric measurements with units, independently of health checks.
A metric carries no history; only its current sample counts. Threshold nodes
derive health by comparing a metric's sample with declared healthy and usable
ranges. A temperature sensor can publish a reading once, while separate threshold nodes
use it to determine thermal health and update readiness. Consumers can display
the reading, its age, and the ranges behind its health without parsing messages.

### Ownership and Updates

A record's owner names the contributor that supplies it, such as a system image
or an application, and replaces or removes it as a whole. To check compatibility
before an update, a controller builds a candidate composition by replacing that owner's
component declarations with the new version's declarations. Components omitted
by the new version are removed from the candidate. Other declarations retain
their requirements, allowing the check to catch dependencies broken by the
update.

Before activation, an update controller checks the candidate composition for
compatibility. After activation, it can require a selected health root to be
healthy before committing the update, meaning accepting it as successful.
An unknown root cannot satisfy that requirement. Other roots can report
concerns that do not block a commit. The controller determines retry and
rollback policy.

### Discovery and Transport

On Linux, contributors publish component declarations, health node definitions,
and statuses in separate standard directories. Readers discover and combine
these records, with directory configuration adding sources such as independently
managed applications. Applications can ship their requirements and health check
definitions alongside their software for update and monitoring tools to discover.
A contributor publishes files without needing a connection to each consumer.

Remote management tools can use a health snapshot to display a device's health
and the checks behind it. A snapshot carries the graph, its statuses, and a
clock sample from the observed system. That sample relates time within a boot
to a wall-clock timestamp, letting the receiver estimate when a check ran. The
snapshot describes health at capture time.

## Specification Reference

### Components and Compatibility

A component is one participant in a system composition. Its
`ComponentDeclaration` has an `id` and optional `version`, `owner`, `provides`,
`claims`, `requires`, and `conflicts` fields. The [owner](#owners-and-update-candidates)
identifies record ownership. Component IDs are opaque, case-sensitive strings
and must be unique within a composition.

A capability describes presence, a version, or an exact string value. A selector
matches capabilities by ID and, optionally, a version requirement or value.
Capabilities appear in `provides`; selectors appear in `requires` and
`conflicts`. A capability or selector cannot specify both a version and a value.
A claim identifies a resource that only one component may own. Every component
also implicitly provides its own ID and version as a capability.

This application requires a runtime capability and claims a TCP port:

```json
{
  "id": "example.app",
  "owner": "rugix.apps.example",
  "version": "2.1",
  "requires": [{"id": "example.runtime", "version": ">=3,<4"}],
  "claims": [{"id": "tcp.port.8080"}]
}
```

A separate platform component can provide the required capability:

```json
{
  "id": "example.platform",
  "provides": [{"id": "example.runtime", "version": "3.2"}]
}
```

Compatibility checking uses all implicit and explicit capabilities:

- Each requirement must match at least one capability.
- Each conflict must match no capability, including the declaring component's.
- A claim can belong to only one component. Repeated claims within one declaration
  count once.
- Multiple components may provide the same capability.

An ID-only selector matches any capability with that ID. A version selector
requires a versioned capability, even for `*`. A value selector requires exact
string equality.

A `CompatibilityReport` contains `consistent` and a list of `problems`.
Unsatisfied requirements, matching conflicts, duplicate claims, and duplicate
component IDs make the composition inconsistent. Invalid version syntax or a
capability or selector with both a version and a value cause an evaluation error.

#### Diagnostic Ordering

Problems are reported as duplicate component IDs, then duplicate claims, then
selector problems. Duplicate ID and claim problems are sorted by Unicode scalar
value. Selectors follow component declaration order, with requirements before
conflicts for each component. Matching capabilities follow provider declaration
order, with each provider's implicit capability preceding its explicit ones.

#### Versions and Requirements

Versions follow Rugix's version format (`anyver` 0.1): one or more numeric, dot-separated
release components, an optional `-` prerelease, and optional `+` build metadata.
Release components and numeric prerelease identifiers must fit in an unsigned
64-bit integer. Prerelease and build identifiers are nonempty, dot-separated
ASCII letters, digits, or hyphens. Leading zeroes are accepted.

Trailing zero release components and build metadata do not affect comparison.
Prereleases precede releases; numeric prerelease identifiers precede text
identifiers, which compare by ASCII order. A shorter prerelease prefix precedes
a longer one.

Requirements are comma-separated conjunctions of `=`, `==`, `!=`, `>`, `>=`,
`<`, or `<=` comparators. A bare version means equality; a sole `*` matches all
versions. Outer whitespace is ignored. Prereleases use the same comparisons
without implicit exclusion. SemVer shorthand such as `^` and `~` is not accepted.

### Metrics and Thresholds

A metric is a named numeric measurement, such as CPU temperature, free
disk space, or battery charge. A `MetricDefinition` declares its ID, unit, optional
label, associated components, owner, and evidence constraints. A separate
`MetricSample` supplies the measurement. Metrics can be displayed without a
health policy, and several threshold nodes can reference the same metric.

Metric IDs are opaque, case-sensitive strings, unique within the metric collection.
They are independent of component and health node IDs. For example, a CPU sensor
can declare:

```json
{
  "id": "cpu.temperature",
  "owner": "platform.sensors",
  "label": "CPU temperature",
  "unit": "celsius",
  "maxAgeMs": 10000,
  "expectedTag": "sensor-v1"
}
```

Its service publishes a separate sample:

```json
{
  "id": "cpu.temperature",
  "owner": "platform.sensors",
  "value": 72.5,
  "bootId": "e09a482c-4a41-4f95-891e-042af3d3c4a4",
  "observedAtMs": 42000,
  "maxAgeMs": 5000,
  "tag": "sensor-v1"
}
```

Sample identity and freshness follow the health status rules: the boot must
match, the sample cannot be in the future, and its age must be strictly less
than its positive validity duration. A definition's `maxAgeMs` and a threshold
rule's `maxAgeMs` can each shorten that duration. A sample must match the
definition's owner and expected tag when specified. The health node may have a
different owner, allowing another contributor to define a policy for the sensor.
Sample messages are optional and are preserved in fresh threshold results.

#### Numbers and Units

Measurements and threshold endpoints use finite IEEE 754 binary64 (`f64`)
numbers, serialized as JSON numbers. Negative and fractional values are
supported. NaN and infinities are invalid. Large byte counts are approximate:
above 2⁵³ bytes, not every individual byte can be represented.

Unit names use kebab-case. Every sample and threshold endpoint uses the
referenced definition's unit:

| Unit | Meaning | Example Display |
| --- | --- | --- |
| `celsius` | Degrees Celsius | 72.5 °C |
| `bytes` | Bytes | 2.4 GiB |
| `seconds` | Seconds | 150 ms |
| `ratio` | Dimensionless fraction; 1 means 100% | 83% |
| `count` | Number of items or events | 12 |
| `bytes-per-second` | Bytes per second | 8.2 MiB/s |
| `hertz` | Cycles per second | 1.2 GHz |
| `volts` | Electric potential | 5 V |
| `amperes` | Electric current | 250 mA |
| `watts` | Power | 12 W |
| `revolutions-per-minute` | Rotational speed | 2400 RPM |

Units describe the measurement. Consumers choose display precision, prefixes,
and conversions, such as Fahrenheit or binary byte prefixes. A ratio is not
implicitly restricted to 0–1. Gauge display bounds are separate from health
thresholds.

#### Threshold Rules

A threshold health node references one metric and declares a desired `healthy`
range and a minimum `usable` range. For example:

```json
{
  "id": "cpu.thermal-health",
  "owner": "platform.policy",
  "rule": {
    "type": "threshold",
    "metric": "cpu.temperature",
    "healthy": {
      "upper": {"value": 75, "inclusive": true}
    },
    "usable": {
      "upper": {"value": 90, "inclusive": true}
    }
  }
}
```

A range has optional `lower` and `upper` bounds. Each bound explicitly states
whether equality is included. An omitted bound is unbounded; `{}` accepts every
finite value. Ranges must be nonempty, and the healthy range must be a subset of
the usable range, including endpoint inclusion.

A fresh sample inside the healthy range produces `healthy`. Otherwise, a
sample inside the usable range produces `degraded`, and one outside it
produces `unhealthy`. In the example, 75 °C is healthy, 90 °C is degraded, and
anything above 90 °C is unhealthy. Lower bounds can express minimum free space,
and two bounds can express acceptable voltage ranges.

Missing, expired, future, wrong-boot, wrong-owner, or wrong-tag samples
produce `unknown`. A threshold result has no health-node inputs; its source metric
is identified by its rule. It contributes to aggregate health like an observed
node. Writers must not supply a `HealthStatus` for a threshold node.

Duplicate metric definitions or samples, missing referenced definitions,
non-finite numbers, zero validity durations, empty or reversed ranges, and
healthy ranges outside usable ranges are evaluation errors. Samples without
a current definition are ignored after validation. Unreferenced metrics remain
valid for display. Callers supply one current sample per metric; evaluators do
not select from a history.

Threshold evaluation is stateless. Debouncing, hysteresis, and historical
aggregation remain the observer's responsibility. A UI may retain a last known
measurement, but it must show its stale state separately from current health.

#### Metric Transport

`HealthSnapshot` and `RuntimeSnapshot` optionally carry `metrics` and
`metricSamples`. Omitted collections mean empty collections. A health
snapshot must include every definition referenced by its threshold nodes.
The clock sample also correlates metric sample times with wall time, using
the same arithmetic as health status timestamps. Transport preserves evidence
age and does not renew validity.

Readers that predate threshold rules cannot evaluate graphs containing them. In Rust, new optional record fields and
the added health rule variant require updates to struct literals and exhaustive
matches.

### Health Graphs

A health graph is a directed acyclic graph of named `HealthNode` definitions.
Nodes have opaque, case-sensitive string IDs, represented by `HealthNodeId` in
the schema. An observed node receives a `HealthStatus` record from a service.
A threshold node derives health from a numeric value and its declared ranges.
An aggregate node evaluates other nodes, named by its `inputs`. Roots are nodes that no other
node uses as an input. Separate roots can describe different system functions or
policies, such as availability and readiness to commit an update (accept it as
successful).

A node can have an `owner`, a label, and associated component IDs. Component
associations do not create health dependencies. A node without a `rule` is an
observed node without age or tag constraints, such as `{"id": "database"}`. Node definitions and status
records are stored separately, so a check remains defined when its observer
stops publishing.

#### States and Results

| State | Meaning |
| --- | --- |
| `healthy` | Desired criteria are confirmed. |
| `degraded` | Minimum criteria are confirmed; desired criteria are unconfirmed or unmet. |
| `unhealthy` | Minimum criteria are confirmed to be unmet. |
| `unknown` | Evidence does not determine whether minimum criteria are met. |

A `HealthReport` contains evaluated nodes in declaration order. Each result
inherits its node's owner and includes its state. Healthy and degraded states
confirm minimum criteria, unhealthy confirms they are unmet, and unknown leaves
them unresolved. A result's `inputs` name its direct dependencies; observed and
threshold nodes have no inputs. Its optional `message` carries the observer's
explanation or the reason a status evaluated to unknown.

#### Observed Nodes and Status Records

An observed node defines the check and any constraints on its evidence:

```json
{
  "id": "application",
  "owner": "rugix.apps.example",
  "rule": {"type": "observed", "maxAgeMs": 10000, "expectedTag": "profile-a"}
}
```

A service reports the check's result in a separate record:

```json
{
  "id": "application",
  "owner": "rugix.apps.example",
  "state": "healthy",
  "bootId": "e09a482c-4a41-4f95-891e-042af3d3c4a4",
  "observedAtMs": 42000,
  "maxAgeMs": 5000,
  "tag": "profile-a",
  "message": "Serving requests"
}
```

On Linux, `bootId` is `/proc/sys/kernel/random/boot_id` and `observedAtMs` is
`CLOCK_BOOTTIME` in milliseconds, including suspended time. Evaluation takes a
`HealthContext` containing the current `bootId` and boot-time value, `nowMs`.

A status is fresh only in that boot, at or after its observation time, and for
strictly less than its positive `maxAgeMs`. An observed rule's optional
`maxAgeMs` must also be positive and can shorten that lifetime. Missing, expired,
future, wrong-boot, wrong-owner, and wrong-tag statuses evaluate to `unknown`
with an explanation. A status must match its node's owner when the node
specifies one. In the example, the status expires at boot-time millisecond 47000:
its 5000 ms lifetime is shorter than the node's 10000 ms limit.

An observed rule's optional `expectedTag` requires an equal `tag` on the status.
Tags are opaque, case-sensitive labels. Changing the expected tag rejects
records from a previous check configuration or instance. Without an expected
tag, tags do not affect evaluation.

Observation times refer to the actual check. Rewriting or reading a file does
not extend its lifetime, and wall-clock changes do not affect freshness.

#### Aggregate Nodes

An aggregate declares `inputs`, an optional minimum `required` count of healthy
or degraded inputs, and an optional desired `healthy` count. Each input is a
record with a referenced `node` ID and optional `required` and `optional` flags,
which default to false. An input must not set both flags. Node IDs must be
distinct.

An optional input only affects the desired criteria: the aggregate is healthy
only if every optional input is healthy, but an optional input never makes the
aggregate unhealthy or unknown. The counts apply to the non-optional inputs. An
omitted `healthy` count means all non-optional inputs, and an omitted `required`
count equals the `healthy` count, so an aggregate without counts requires all
non-optional inputs to be healthy. The resolved counts must satisfy
`1 <= required <= healthy <= N`, where `N` is the number of non-optional inputs.

Existing declarations must convert string inputs such as `"storage"` to
`{"node": "storage"}`, even when no input is individually required.

```json
{
  "id": "system.commit",
  "owner": "rugix.system",
  "rule": {
    "type": "aggregate",
    "inputs": [{"node": "storage"}, {"node": "application"}]
  }
}
```

Both input nodes must exist in the graph. Without individually required inputs,
omitting both counts expresses all-of; setting `healthy` to one expresses
any-of. Setting `required` below `healthy` expresses redundancy that
can tolerate some failures.

An input with `required: true` is mandatory. It must be healthy or degraded for
the aggregate to meet its minimum criteria, and healthy for the aggregate to be
healthy. For example, a database can be mandatory while workers are redundant:

```json
{
  "id": "service",
  "rule": {
    "type": "aggregate",
    "inputs": [
      {"node": "database", "required": true},
      {"node": "worker-a"},
      {"node": "worker-b"}
    ],
    "required": 2,
    "healthy": 3
  }
}
```

The database counts toward both totals. Minimum functioning requires the
database and at least one worker; full health requires all three to be healthy.
Healthy workers cannot compensate for an unhealthy database. A degraded database
prevents full health even if the desired healthy count is otherwise satisfied.

An input with `optional: true` suits a function whose failure the aggregate can
tolerate. For example, a telemetry service that buffers data locally remains
usable while its upload fails:

```json
{
  "id": "telemetry",
  "rule": {
    "type": "aggregate",
    "inputs": [
      {"node": "database"},
      {"node": "plc"},
      {"node": "upload", "optional": true}
    ]
  }
}
```

Evaluation counts three kinds of evidence among the non-optional inputs:

| Count | Inputs Included |
| --- | --- |
| `H` | State is `healthy`. |
| `M` | State is `healthy` or `degraded`, confirming minimum criteria. |
| `P` | State is `healthy`, `degraded`, or `unknown`, so minimum criteria may be met. |

Apply these rules in order to determine an aggregate's state:

1. If any mandatory input is unhealthy or `P < required`, the state is `unhealthy`.
2. If any mandatory input is unknown or `M < required`, the state is `unknown`.
3. If every mandatory and every optional input is healthy and `H >= healthy`, the state is `healthy`.
4. Otherwise, the state is `degraded`.

A confirmed failure takes precedence over uncertainty. An unknown mandatory
input produces `unknown` unless another mandatory input is unhealthy or the
minimum count is already impossible.

For two observed inputs without mandatory flags and with
`required: 1, healthy: 2`, the results include:

| Input States | Aggregate State |
| --- | --- |
| healthy, healthy | healthy |
| healthy, unhealthy | degraded |
| healthy, unknown | degraded |
| degraded, unknown | degraded |
| unhealthy, unknown | unknown |
| unhealthy, unhealthy | unhealthy |

A healthy input confirms the minimum and desired criteria; a degraded input
confirms only the minimum. An unknown input contributes only to `P`. These same
rules apply when an input is itself an aggregate. Its state contains all the
information required by its parent. Result `inputs` list dependency node IDs;
the mandatory flags belong to the aggregate definition.

#### Graph Validation

Missing input nodes, duplicate node IDs or referenced input IDs, inputs that are both
required and optional, invalid thresholds, cycles,
duplicate statuses, statuses supplied for aggregate or threshold nodes, and zero `maxAgeMs`
values are evaluation errors. Statuses without a current node are ignored after
validation. An invalid graph or status prevents evaluation of the whole graph.

#### Commit Decisions

A commit controller can select `system.commit` and require it to be `healthy`.
Other roots can report concerns that do not block a commit. The controller
chooses grace periods, retries, and rollback policy. An unknown root cannot
satisfy a healthy commit requirement. A current decision requires current
boot-time evidence; a previously captured snapshot describes health at capture
time.

#### Clock Correlation

A `HealthSnapshot` contains node definitions in `nodes`, original status records
in `statuses`, and a `HealthClockSample` in `clock`. Collect the records first,
then sample the clock on the observed system. The sample pairs a boot-time
context with a UTC wall-clock timestamp from that system. A receiver's local
clock cannot supply this correlation. Clock timestamps use RFC 3339 in UTC,
ending in `Z`, and may include fractional seconds. Sampling uncertainty and
millisecond rounding limit the precision of reconstructed observation times.

For example, this clock sample correlates boot-time millisecond 45000 with
`2026-10-14T12:00:00Z`:

```json
{
  "context": {
    "bootId": "e09a482c-4a41-4f95-891e-042af3d3c4a4",
    "nowMs": 45000
  },
  "timestamp": "2026-10-14T12:00:00Z"
}
```

For a status from the same boot, its estimated wall time is
`clock.timestamp - (clock.context.nowMs - status.observedAtMs) milliseconds`.
The earlier status at millisecond 42000 maps to `2026-10-14T11:59:57Z`.

For display, observation timestamps have millisecond precision within years 0001
through 9999. A wrong boot, future observation, or result outside that range has
no timestamp. An expired status retains its original observation time.

Wall-clock corrections can shift these estimates; a clock sample cannot recover
historical adjustments. Transport preserves observation times and `maxAgeMs`.
Evaluating with `snapshot.clock.context` determines health at capture time;
transport delays and subsequent changes can make that evidence stale.

### Owners and Update Candidates

The `owner` field names the system image, app, or other contributor that owns a
component, health node, status, metric definition, or metric sample. Evaluated
health results inherit their node's owner. Owner names are opaque, case-sensitive strings matched by exact
equality. For Rugix integration, use `rugix.system` for a system image and
`rugix.apps.<name>` for an app. These are naming conventions; readers do not
infer owners from directory paths.

Owners do not scope IDs or affect capability matching. Several components or
nodes may share an owner, while their IDs remain unique within their respective
collections. Records without an owner remain valid.

To check an update candidate, remove records with the owner being replaced and
add the new declarations. Removing an owner also removes components omitted by
the replacement, even when their IDs have no direct replacements. Preserve other
records and their references to removed records, so compatibility checking can
detect unmet requirements. Evaluate runtime health after activation, using fresh
status records from the running system.

### File Discovery

Each regular `.json` file contains one component declaration, health node, status
record, metric definition, metric sample, or directory configuration, according
to its directory. Writers must
omit optional fields when they are unset. Readers must ignore unknown record
fields for forward compatibility.

| Collection | Standard Directories |
| --- | --- |
| Components | `/usr/lib/system-runtime/components`, `/etc/system-runtime/components`, `/run/system-runtime/components` |
| Health nodes | `/usr/lib/system-runtime/health/nodes`, `/etc/system-runtime/health/nodes`, `/run/system-runtime/health/nodes` |
| Health statuses | `/run/system-runtime/health/status` |
| Metric definitions | `/usr/lib/system-runtime/metrics/definitions`, `/etc/system-runtime/metrics/definitions`, `/run/system-runtime/metrics/definitions` |
| Metric samples | `/run/system-runtime/metrics/samples` |
| Directory configurations | `/usr/lib/system-runtime/directories`, `/etc/system-runtime/directories`, `/run/system-runtime/directories` |

Readers combine contributions without filename overrides. They process sources
in the listed order, with lexical ordering of directory matches and recursive
lexical traversal within each source. Overlapping sources read each canonical
file path once per collection. Duplicate IDs across files remain evaluation
errors.

Missing directories are ignored. Present non-directories, broken source
symlinks, and read or parse errors are reported. An explicit source may be a
symlink. Readers resolve each source to its physical directory before traversal,
so changing an activation symlink cannot redirect files already discovered.
Nested symlinks and files with other extensions are skipped.

A `DirectoryConfig` appends absolute paths through `components`, `healthNodes`,
`healthStatuses`, `metrics`, and `metricSamples`.

A whole `*` path segment matches real directories at that level. Matched directory
symlinks are skipped; explicit segments such as `current` may follow symlinks.
Partial wildcards, `**`, relative paths, and `..` segments are rejected. Configuration files are reloaded on every load and cannot
add other configuration directories.

Writers create a temporary file with an unrecognized extension in the same
directory, then atomically rename it over the final file. There is no transaction
across files. Consumers retry reads during transitions and evaluate a fresh
snapshot before acting. One observer owns each status ID; shared storage does
not authenticate publishers.

#### Additional Directories and Rugix Apps

Convert Rugix declarations stored as TOML to `.json` files before using these
configurations; TOML files are ignored by discovery. This configuration in
`/etc/system-runtime/directories/rugix.json` includes Rugix component directories:

```json
{
  "components": [
    "/usr/lib/rugix/components",
    "/etc/rugix/components",
    "/run/rugix/components"
  ]
}
```

Rugix Ctrl also creates a `rugix.host` component in memory; integrations must
supply it separately when requirements refer to its `host.arch` or `rugix.ctrl`
capabilities. Existing files without owners need
attribution before replacement by owner.

In Rugix Apps, a generation is one installed version of an app's files. On
installations where Rugix Ctrl exposes the activated generation through a
`current` symlink, `/etc/system-runtime/directories/apps.json` can discover all
active apps:

```json
{
  "components": ["/run/rugix/state/apps/*/current/.rugix/components"],
  "healthNodes": ["/run/rugix/state/apps/*/current/.rugix/health/nodes"],
  "healthStatuses": ["/run/example/health/status"]
}
```

Use `/var/lib/rugix/apps` instead of `/run/rugix/state/apps` when Rugix state
management is absent. The status directory belongs to a publishing service;
choose its path to match that service. Patterns are expanded on each load, so
new apps and activation changes need no configuration update.

The `current` link retains the previous generation's contributions while a
workload stops, starts, or switches. It changes atomically after successful
activation. Failed activation with successful rollback retains the previous
generation. Completed deactivation removes the link; initial activation has no
previous generation to expose.

### Data Models and Encoding

The [schemas](schemas/) use Sidex, a language for defining data models. JSON
field names use camelCase. Unsigned 64-bit integers above JavaScript's safe
integer range are encoded as decimal strings.

Shared [conformance fixtures](tests/fixtures/) cover compatibility and health
evaluation across implementations.

## Implementations

| Language | Package and Documentation |
| --- | --- |
| Rust | [`system-runtime-spec`](crates/libs/system-runtime/README.md) |
| Python | [`system-runtime-spec`](python/README.md) |
| TypeScript | [`system-runtime-spec`](typescript/README.md) |

All three packages provide generated models, compatibility checking, and health
evaluation, including numeric thresholds. Rust and Python also implement file discovery. Package READMEs cover
installation and API usage.

## Development

Generated models and JSON Schemas are included in the repository. To regenerate
them and run formatting, linting, type, conformance, and package build checks:

```sh
mise install
mise run generate
mise run check
```

CI runs the same checks. See [Releasing the Packages](RELEASING.md) for registry
publication.

## License

Available under either the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE).
