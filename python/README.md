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

Import the package as `system_runtime_spec`. Records are JSON objects (`dict`), so that snapshots
carry them unchanged and records of newer minor versions keep working. Records may also be the
generated Pydantic models in `system_runtime_spec.models`.

## Read and Check a System

```python
from system_runtime_spec import (
    check_components,
    current_context,
    evaluate_health,
    read_records,
)

records = read_records("/")
compatibility = check_components(records.components, records.diagnostics)
health = evaluate_health(records, current_context())
print(compatibility.consistent)
for node in health.nodes:
    print(node.id, node.usable.root, node.healthy.root)
```

`read_records` reads the standard directories and the directories that directory configurations
add, and applies overrides. Files that cannot be read or do not contain a valid record are skipped
and reported as diagnostics. Checks and evaluation take the diagnostics into account: an error
makes a composition inconsistent, and a decision should not rely on a health report with errors
about health nodes or directory configurations.

## Check an Update

```python
from system_runtime_spec import candidate_composition, check_components, read_records

records = read_records("/")
components, diagnostics = candidate_composition(
    records.components,
    records.diagnostics,
    ["rugix.system"],
    [{"id": "os", "owner": "rugix.system", "version": "2"}],
)
print(check_components(components, diagnostics).consistent)
```

## Send Records as a Snapshot

`make_snapshot` creates a snapshot with the original records, and `read_snapshot` reads one on
the receiving system. Take the clock sample after reading the records:

```python
import json

from system_runtime_spec import (
    current_clock_sample,
    evaluate_health,
    make_snapshot,
    read_records,
    read_snapshot,
)

records = read_records("/")
payload = json.dumps(make_snapshot(records, current_clock_sample()))
received = read_snapshot(json.loads(payload))
report = evaluate_health(received.records, received.clock.context)
```

Evaluating a snapshot gives health at the time of its clock sample. `estimate_timestamp` places
evidence in wall-clock time for display.

## License

Available under either the [MIT License](LICENSE-MIT) or the
[Apache License 2.0](LICENSE-APACHE).
