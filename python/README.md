# System Runtime for Python

Use `system-runtime` to load component and health files on Linux, check component
compatibility, and evaluate health graphs. Its models implement the
[System Runtime Specification](https://github.com/rugix/system-runtime-spec).

## Installation

Python 3.12 or later is required. Install from a local checkout:

```sh
python -m pip install /path/to/system-runtime-spec/python
```

The distribution is named `system-runtime`; import it as `system_runtime`.

## Read and Check a System

`RuntimeDirectories.standard()` loads the specification's standard directories
and extension configurations. The returned `RuntimeSnapshot` contains components,
health node definitions, and status records.

```python
from system_runtime import (
    RuntimeDirectories,
    check_components,
    current_context,
    evaluate_health,
)

snapshot = RuntimeDirectories.standard().load()
compatibility = check_components(snapshot.components)
health = evaluate_health(
    snapshot.health_nodes, snapshot.health_statuses, current_context()
)
print(compatibility.model_dump_json(), health.model_dump_json())
```

`current_context()` reads the Linux boot ID and `CLOCK_BOOTTIME` for freshness
checks. For records from another system, supply that system's `HealthContext`.
Invalid declarations and discovery failures raise exceptions; a valid but
incompatible composition returns a report with `consistent == False`.

## Add Directories

Use extension files as described in the
[file discovery specification](https://github.com/rugix/system-runtime-spec#file-discovery),
or extend the sources in code:

```python
from system_runtime import RuntimeDirectories
from system_runtime.filesystem import DirectoryConfig

directories = RuntimeDirectories.standard()
directories.extend(DirectoryConfig(components=["/opt/example/components"]))
snapshot = directories.load()
```

Use `RuntimeDirectories()` to start with empty source lists.
`RuntimeDirectories.standard("/mnt/system")` relocates absolute source paths for
tests or alternate installations. Absolute symlink targets still resolve against
the host's root, rather than the supplied root.

## Check an Update Candidate

An origin groups records owned by a system image, app, or other contributor.
`without_origin` returns a snapshot with new lists that exclude records owned by
that exact origin. Other records keep their requirements and references. Retained
record objects are shared with the original snapshot; copy a record before
modifying its fields.

```python
from system_runtime import RuntimeDirectories, check_components, without_origin
from system_runtime.models.component import ComponentDeclaration

current = RuntimeDirectories.standard().load()
candidate = without_origin(current, "rugix.system")
candidate.components.append(
    ComponentDeclaration(id="example.system", origin="rugix.system", version="2.0")
)
report = check_components(candidate.components)
print(report.consistent)
```

The current records must already carry origins. Discovery does not infer
ownership from paths; attribute imported records before assembling the snapshot.
This operation changes only the in-memory candidate. After activation, evaluate
health using fresh records from the running system.

### Import Records Without Origins

For existing records without origins, load each set of directories once before
constructing a candidate. This example builds a snapshot of components from
system-image and device-local directories without loading directory configuration
files:

```python
from system_runtime import RuntimeDirectories

current = RuntimeDirectories(
    components=["/etc/rugix/components", "/run/rugix/components"],
).load()
image = RuntimeDirectories(components=["/usr/lib/rugix/components"]).load()
for component in image.components:
    component.origin = "rugix.system"
current.components.extend(image.components)
```

Use this prepared `current` in the replacement example instead of loading it with
`standard()`. Before evaluating the complete system, add app contributions and
any components from standard directories that the system relies on. Load health
nodes and statuses separately for health evaluation. Read each set of directories
once; combining this import with a standard load of the same files would
duplicate records. The assignments change only in-memory records. Device-local
records without an origin survive replacement.

## Transport a Health Snapshot

Collect records first, then sample the observed system's clock:

```python
from system_runtime import RuntimeDirectories, current_clock_sample, status_timestamp
from system_runtime.models.health import HealthSnapshot

records = RuntimeDirectories.standard().load()
snapshot = HealthSnapshot(
    clock=current_clock_sample(),
    nodes=records.health_nodes,
    statuses=records.health_statuses,
)
payload = snapshot.model_dump_json()
received = HealthSnapshot.model_validate_json(payload)
for status in received.statuses:
    print(status.id, status_timestamp(status, received.clock))
```

`current_clock_sample()` samples UTC between two boot-time reads and pairs it
with their midpoint. `status_timestamp` returns an estimated UTC `datetime` for
display, or `None` for a wrong boot, future observation, or unsupported timestamp
range. Sampling uncertainty and wall-clock corrections limit the estimate.

Evaluating with `received.clock.context` gives health at capture time. Obtain
current evidence before making an update decision.

## Models

Sidex generates Pydantic models in `system_runtime.models.component`,
`system_runtime.models.health`, and `system_runtime.models.filesystem`. Python
attributes use snake_case, such as `boot_id`; serialized fields use names such as
`bootId`. Use `model_validate` or `model_validate_json` to parse records and
`model_dump_json()` to serialize them in the specified wire format. Unset
optional fields are omitted. Health node IDs are string type aliases.

See the [specification](https://github.com/rugix/system-runtime-spec) for wire
formats, version matching, and health semantics.

## License

Available under either the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE).
