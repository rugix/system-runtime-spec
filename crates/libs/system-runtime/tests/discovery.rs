use std::path::Path;

use system_runtime::filesystem::DirectoryConfig;
use system_runtime::health::{HealthContext, HealthNode, HealthState};
use system_runtime::{
    RuntimeDirectories, check_components, evaluate_health, evaluate_health_with_values,
};

/// Copy a shared filesystem use case into an isolated test root.
fn copy_tree(source: &Path, destination: &Path) {
    std::fs::create_dir_all(destination).unwrap();
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// JSON contributions reload without refreshing evidence or accepting other formats.
#[test]
fn directory_integration() {
    let root = tempfile::tempdir().unwrap();
    copy_tree(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../tests/fixtures/runtime")
            .as_path(),
        root.path(),
    );
    let mut directories = RuntimeDirectories::standard(root.path());
    directories.extend(&DirectoryConfig {
        values: None,
        value_observations: None,
        components: Some(vec!["/apps/7/components".into()]),
        health_nodes: None,
        health_statuses: None,
    });
    let snapshot = directories.load().unwrap();
    assert_eq!(
        snapshot
            .components
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        ["runtime", "app"]
    );
    assert!(check_components(&snapshot.components).unwrap().consistent);
    assert_eq!(snapshot.health_nodes.len(), 2);
    assert_eq!(snapshot.health_statuses.len(), 1);
    let mut context = HealthContext {
        boot_id: "boot".into(),
        now_ms: 100,
    };
    assert_eq!(
        evaluate_health(&snapshot.health_nodes, &snapshot.health_statuses, &context)
            .unwrap()
            .nodes[0]
            .state,
        HealthState::Healthy
    );
    let value_node: HealthNode = sidex::formats::json::from_str(
        r#"{
        "id":"storage", "rule":{"rule":"threshold", "value":"storage.free",
        "healthy":{"lower":{"value":104857600,"inclusive":true}},
        "usable":{}}}"#,
    )
    .unwrap();
    let values = snapshot.values.as_deref().unwrap();
    let observations = snapshot.value_observations.as_deref().unwrap();
    assert_eq!(values.len(), 1);
    assert_eq!(observations.len(), 1);
    assert_eq!(
        evaluate_health_with_values(
            std::slice::from_ref(&value_node),
            &[],
            values,
            observations,
            &context
        )
        .unwrap()
        .nodes[0]
            .state,
        HealthState::Healthy
    );
    context.now_ms = 110;
    assert_eq!(
        evaluate_health(&snapshot.health_nodes, &snapshot.health_statuses, &context)
            .unwrap()
            .nodes[0]
            .state,
        HealthState::Unknown
    );
    assert_eq!(
        evaluate_health_with_values(&[value_node], &[], values, observations, &context)
            .unwrap()
            .nodes[0]
            .state,
        HealthState::Unknown
    );
    std::fs::write(
        root.path().join("run/system-runtime/directories/apps.json"),
        r#"{"components":["/apps/8/components"]}"#,
    )
    .unwrap();
    assert_eq!(
        directories
            .load()
            .unwrap()
            .components
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        ["runtime", "app", "inactive"]
    );
}

/// Explicit symlinks work, nested links are skipped, and invalid contributions fail.
#[cfg(unix)]
#[test]
fn directory_symlinks_and_errors() {
    let root = tempfile::tempdir().unwrap();
    let source = root.path().join("source");
    std::fs::create_dir(&source).unwrap();
    std::fs::write(source.join("ok.json"), r#"{"id":"ok"}"#).unwrap();
    std::os::unix::fs::symlink(source.join("ok.json"), source.join("nested.json")).unwrap();
    std::os::unix::fs::symlink(&source, root.path().join("active")).unwrap();
    let mut directories = RuntimeDirectories::new(root.path());
    directories.components = vec!["/active".into(), "/source".into()];
    assert_eq!(directories.load().unwrap().components.len(), 1);
    std::fs::write(source.join("bad.json"), "id = \"bad\"\n").unwrap();
    assert!(
        directories
            .load()
            .unwrap_err()
            .to_string()
            .contains("bad.json")
    );
    for invalid in [
        "relative",
        "/source/../source",
        "/source/ok.json",
        "/source/**",
        "/sou*",
    ] {
        directories.components = vec![invalid.into()];
        assert!(directories.load().is_err(), "{invalid}");
    }
}

/// One static directory extension follows all active app generations and their health.
#[cfg(unix)]
#[test]
fn app_generation_patterns() {
    let root = tempfile::tempdir().unwrap();
    let app = root.path().join("apps/example");
    for generation in 1..=2 {
        let dir = app.join(format!("generations/{generation}/.rugix"));
        for collection in ["components", "health/nodes", "health/status"] {
            std::fs::create_dir_all(dir.join(collection)).unwrap();
        }
        std::fs::write(
            dir.join("components/app.json"),
            format!(r#"{{"id":"app.{generation}"}}"#),
        )
        .unwrap();
        std::fs::write(
            dir.join("health/nodes/app.json"),
            format!(r#"{{"id":"app.{generation}","rule":{{"rule":"observed"}}}}"#),
        )
        .unwrap();
        std::fs::write(
            dir.join("health/status/app.json"),
            format!(r#"{{"id":"app.{generation}","state":"healthy","bootId":"boot","observedAtMs":100,"maxAgeMs":10}}"#),
        ).unwrap();
    }
    std::fs::create_dir_all(root.path().join("apps/inactive")).unwrap();
    let other = root.path().join("unrelated/current/.rugix/components");
    std::fs::create_dir_all(&other).unwrap();
    std::fs::write(other.join("app.json"), r#"{"id":"unrelated"}"#).unwrap();
    std::os::unix::fs::symlink(
        root.path().join("unrelated"),
        root.path().join("apps/alias"),
    )
    .unwrap();
    let configuration = root.path().join("etc/system-runtime/directories");
    std::fs::create_dir_all(&configuration).unwrap();
    std::fs::write(
        configuration.join("apps.json"),
        r#"{
  "components": ["/apps/*/current/.rugix/components", "/apps/example/current/.rugix/components"],
  "healthNodes": ["/apps/*/current/.rugix/health/nodes"],
  "healthStatuses": ["/apps/*/current/.rugix/health/status"]
}"#,
    )
    .unwrap();
    let directories = RuntimeDirectories::standard(root.path());
    for generation in 1..=2 {
        let temporary = app.join("next");
        std::os::unix::fs::symlink(format!("generations/{generation}"), &temporary).unwrap();
        std::fs::rename(temporary, app.join("current")).unwrap();
        let snapshot = directories.load().unwrap();
        let id = format!("app.{generation}");
        assert_eq!(snapshot.components.len(), 1);
        assert_eq!(snapshot.components[0].id, id);
        assert_eq!(snapshot.health_nodes.len(), 1);
        assert_eq!(snapshot.health_nodes[0].id.as_str(), id);
        assert_eq!(snapshot.health_statuses.len(), 1);
        assert_eq!(snapshot.health_statuses[0].id.as_str(), id);
        assert_eq!(
            evaluate_health(
                &snapshot.health_nodes,
                &snapshot.health_statuses,
                &HealthContext {
                    boot_id: "boot".into(),
                    now_ms: 100
                },
            )
            .unwrap()
            .nodes[0]
                .state,
            HealthState::Healthy
        );
    }
    std::fs::remove_file(app.join("current")).unwrap();
    let snapshot = directories.load().unwrap();
    assert!(snapshot.components.is_empty());
    assert!(snapshot.health_nodes.is_empty());
    assert!(snapshot.health_statuses.is_empty());
}
