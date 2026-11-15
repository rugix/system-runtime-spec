"""Shared cross-language compatibility and health use cases."""

import json
import shutil
from pathlib import Path

import pytest

from system_runtime_spec import RuntimeDirectories, check_components, evaluate_health
from system_runtime_spec.models.component import ComponentDeclaration
from system_runtime_spec.models.filesystem import DirectoryConfig
from system_runtime_spec.models.health import HealthContext, HealthNode, HealthStatus
from system_runtime_spec.models.value import ValueDefinition, ValueObservation

FIXTURES = Path(__file__).resolve().parents[2] / "tests/fixtures"
CASES = json.loads((FIXTURES / "conformance.json").read_text())


@pytest.mark.parametrize("case", CASES["versions"])
def test_versions(case):
    """Version requirements preserve numeric precision and Rugix ordering."""
    components = [
        ComponentDeclaration(id="provider", version=case["version"]),
        ComponentDeclaration.model_validate(
            {
                "id": "consumer",
                "requires": [{"id": "provider", "version": case["requirement"]}],
            }
        ),
    ]
    if case.get("error"):
        with pytest.raises(ValueError):
            check_components(components)
    else:
        assert check_components(components).consistent == case["matches"]


@pytest.mark.parametrize("case", CASES["components"], ids=lambda case: case["name"])
def test_components(case):
    """Whole compositions produce the shared constraint diagnostics."""
    components = [
        ComponentDeclaration.model_validate(value) for value in case["components"]
    ]
    if case.get("error"):
        with pytest.raises(ValueError):
            check_components(components)
    else:
        assert (
            json.loads(check_components(components).model_dump_json()) == case["report"]
        )


@pytest.mark.parametrize("case", CASES["health"], ids=lambda case: case["name"])
def test_health(case):
    """Health evaluation preserves uncertainty, identity, and finite freshness."""
    nodes = [HealthNode.model_validate(value) for value in case["nodes"]]
    statuses = [HealthStatus.model_validate(value) for value in case["statuses"]]
    context = HealthContext.model_validate(case["context"])
    values = [ValueDefinition.model_validate(v) for v in case.get("values", [])]
    samples = [
        ValueObservation.model_validate(v) for v in case.get("valueObservations", [])
    ]
    if case.get("error"):
        with pytest.raises(ValueError):
            evaluate_health(
                nodes, statuses, context, values=values, value_observations=samples
            )
    else:
        assert (
            json.loads(
                evaluate_health(
                    nodes, statuses, context, values=values, value_observations=samples
                ).model_dump_json()
            )
            == case["report"]
        )


def test_directory_integration(tmp_path):
    """Reload JSON contributions, ignoring other formats and retaining evidence age."""
    shutil.copytree(FIXTURES / "runtime", tmp_path, dirs_exist_ok=True)
    directories = RuntimeDirectories.standard(tmp_path)
    directories.extend(DirectoryConfig(components=["/apps/7/components"]))
    snapshot = directories.load()
    assert [component.id for component in snapshot.components] == ["runtime", "app"]
    assert check_components(snapshot.components).consistent
    assert len(snapshot.health_nodes) == 2
    assert len(snapshot.health_statuses) == 1
    context = HealthContext(boot_id="boot", now_ms=100)
    assert (
        evaluate_health(snapshot.health_nodes, snapshot.health_statuses, context)
        .nodes[0]
        .state.root
        == "healthy"
    )
    node = HealthNode.model_validate(
        {
            "id": "storage",
            "rule": {
                "rule": "threshold",
                "value": "storage.free",
                "healthy": {
                    "lower": {
                        "value": 104857600,
                        "inclusive": True,
                    }
                },
                "usable": {},
            },
        }
    )
    assert len(snapshot.values) == len(snapshot.value_observations) == 1

    def measured_state():
        return (
            evaluate_health(
                [node],
                [],
                context,
                values=snapshot.values,
                value_observations=snapshot.value_observations,
            )
            .nodes[0]
            .state.root
        )

    assert measured_state() == "healthy"
    context.now_ms = 110
    assert measured_state() == "unknown"
    assert (
        evaluate_health(snapshot.health_nodes, snapshot.health_statuses, context)
        .nodes[0]
        .state.root
        == "unknown"
    )
    config = tmp_path / "run/system-runtime/directories/apps.json"
    config.write_text(json.dumps({"components": ["/apps/8/components"]}))
    assert [component.id for component in directories.load().components] == [
        "runtime",
        "app",
        "inactive",
    ]


def test_directory_symlinks_and_errors(tmp_path):
    """Read explicit symlinks, skip nested links, and report invalid files."""
    source = tmp_path / "source"
    source.mkdir()
    (source / "ok.json").write_text(json.dumps({"id": "ok"}))
    (source / "nested.json").symlink_to(source / "ok.json")
    (tmp_path / "active").symlink_to(source, target_is_directory=True)
    directories = RuntimeDirectories(root=tmp_path, components=["/active", "/source"])
    assert len(directories.load().components) == 1
    (source / "bad.json").write_text('id = "bad"\n')
    with pytest.raises(ValueError, match="bad.json"):
        directories.load()
    for invalid in ["relative", "/source/../source", "/source/**", "/sou*"]:
        with pytest.raises(ValueError, match="absolute"):
            RuntimeDirectories(root=tmp_path, components=[invalid]).load()
    with pytest.raises(NotADirectoryError):
        RuntimeDirectories(root=tmp_path, components=["/source/ok.json"]).load()


def test_app_generation_patterns(tmp_path):
    """One static extension follows activated app components and health on reload."""
    app = tmp_path / "apps/example"
    for generation in (1, 2):
        directory = app / f"generations/{generation}/.rugix"
        for collection in ("components", "health/nodes", "health/status"):
            (directory / collection).mkdir(parents=True)
        identifier = f"app.{generation}"
        (directory / "components/app.json").write_text(json.dumps({"id": identifier}))
        (directory / "health/nodes/app.json").write_text(
            json.dumps({"id": identifier, "rule": {"rule": "observed"}})
        )
        (directory / "health/status/app.json").write_text(
            json.dumps(
                {
                    "id": identifier,
                    "state": "healthy",
                    "bootId": "boot",
                    "observedAtMs": 100,
                    "maxAgeMs": 10,
                }
            )
        )
    (tmp_path / "apps/inactive").mkdir()
    other = tmp_path / "unrelated/current/.rugix/components"
    other.mkdir(parents=True)
    (other / "app.json").write_text(json.dumps({"id": "unrelated"}))
    (tmp_path / "apps/alias").symlink_to(tmp_path / "unrelated")
    configuration = tmp_path / "etc/system-runtime/directories"
    configuration.mkdir(parents=True)
    (configuration / "apps.json").write_text(
        json.dumps(
            {
                "components": [
                    "/apps/*/current/.rugix/components",
                    "/apps/example/current/.rugix/components",
                ],
                "healthNodes": ["/apps/*/current/.rugix/health/nodes"],
                "healthStatuses": ["/apps/*/current/.rugix/health/status"],
            }
        )
    )
    directories = RuntimeDirectories.standard(tmp_path)
    for generation in (1, 2):
        temporary = app / "next"
        temporary.symlink_to(f"generations/{generation}")
        temporary.replace(app / "current")
        snapshot = directories.load()
        identifier = f"app.{generation}"
        assert [item.id for item in snapshot.components] == [identifier]
        assert [item.id for item in snapshot.health_nodes] == [identifier]
        assert [item.id for item in snapshot.health_statuses] == [identifier]
        assert (
            evaluate_health(
                snapshot.health_nodes,
                snapshot.health_statuses,
                HealthContext(boot_id="boot", now_ms=100),
            )
            .nodes[0]
            .state.root
            == "healthy"
        )
    (app / "current").unlink()
    snapshot = directories.load()
    assert (
        snapshot.components == snapshot.health_nodes == snapshot.health_statuses == []
    )


def test_discovery_pins_generation_before_reading(tmp_path):
    """Changing current after discovery cannot redirect an already collected file."""
    for generation in (1, 2):
        directory = tmp_path / str(generation)
        directory.mkdir()
        (directory / "app.json").write_text(json.dumps({"id": f"app.{generation}"}))
    current = tmp_path / "current"
    current.symlink_to("1")
    directories = RuntimeDirectories(root=tmp_path, components=["/current"])
    files = directories._files(directories.components)
    current.unlink()
    current.symlink_to("2")
    assert json.loads(files[0].read_text()) == {"id": "app.1"}
    assert directories.load().components[0].id == "app.2"
