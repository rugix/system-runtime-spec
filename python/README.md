# System Runtime Specification for Python

`system-runtime-spec` implements the
[System Runtime Specification](https://github.com/rugix/system-runtime-spec/blob/main/SPEC.md)
in Python: it discovers records on Linux, checks the compatibility of compositions and updates,
evaluates health, and sends records to other systems as snapshots.

## Installation

Python 3.12 or later is required:

```sh
python -m pip install system-runtime-spec
```

Import the package as `system_runtime_spec`. `Records` holds the records of a system and the
diagnostics about them. The Pydantic models in `system_runtime_spec.models` are generated from the
specification's schemas.

## Read and Check a System

```python
from system_runtime_spec import Records, current_context

records = Records.discover("/")
compatibility = records.check_compatibility()
health = records.evaluate_health(current_context())
print("consistent:", compatibility.consistent)
for result in health.results:
    print(f"{result.id}: usable {result.usable}, healthy {result.healthy}")
```

`Records.discover` reads the standard directories and the directories that directory
configurations add, and applies overrides. Files that cannot be read or do not contain a valid
record are skipped and reported as diagnostics. Reports take the diagnostics of their part into
account: an error about component declarations makes a composition inconsistent, and a decision
should not rely on a health report with errors about health nodes or directory configurations.

## Add Records in Code

`push` adds a record given as a model, and `push_json` one given as JSON, such as a record
received from another system. Both validate the record and skip it with an error diagnostic if it
is invalid:

```python
from system_runtime_spec import Records
from system_runtime_spec.models.component import (
    CapabilitySelectorDeclaration,
    ComponentDeclaration,
)

records = Records()
records.push(ComponentDeclaration(id="os", version="2.1"))
records.push(
    ComponentDeclaration(
        id="app",
        requires=[CapabilitySelectorDeclaration(id="os", version=">=2")],
    )
)
records.push_json(ComponentDeclaration, {"id": "tool"})
assert records.check_compatibility().consistent
```

## Check an Update

`with_update` builds the composition that an update would produce: it replaces all component
declarations of the given owners with those of the update.

```python
from system_runtime_spec import Records
from system_runtime_spec.models.component import ComponentDeclaration

records = Records.discover("/")
update = Records()
update.push(ComponentDeclaration(id="os", owner="rugix.system", version="2.0"))
candidate = records.with_update(["rugix.system"], update)
print("consistent:", candidate.check_compatibility().consistent)
```

## Send Records as a Snapshot

A `Snapshot` contains the original records, so the receiving system evaluates them as the sending
system read them. Take the clock sample after reading the records:

```python
import json

from system_runtime_spec import (
    Records,
    Snapshot,
    current_clock_sample,
    estimate_timestamp,
)

records = Records.discover("/")
payload = json.dumps(Snapshot(records, current_clock_sample()).to_json())

# On the receiving system:
received = Snapshot.from_json(json.loads(payload))
report = received.records.evaluate_health(received.clock.context)
for status in received.records.health_statuses:
    print(status.id, estimate_timestamp(received.clock, status))
```

`Snapshot` takes `components=False` or `health=False` to leave out a part. `Snapshot.from_json`
raises a `SnapshotError` for snapshots of an unsupported version or with an invalid structure.
Evaluating a snapshot gives health at the time of its clock sample, and `estimate_timestamp`
places evidence in wall-clock time for display.

`Records.discover`, `current_context`, and `current_clock_sample` are available on Linux only.

## License

Available under either the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE).
