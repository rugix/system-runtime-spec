use std::process::Output;

use system_runtime_spec::types::component::{CompatibilityReport, ComponentDeclaration};
use system_runtime_spec::types::health::{HealthContext, HealthReport};

/// A system root below a temporary directory.
struct System {
    root: tempfile::TempDir,
}

impl System {
    /// A system whose app requires a capability that the system image provides, with an
    /// aggregate health node over an unhealthy and an unreported node.
    fn new() -> Self {
        let system = Self {
            root: tempfile::tempdir().unwrap(),
        };
        system.write(
            "usr/lib/system-runtime/components/os.json",
            r#"{"id": "os", "owner": "rugix.system", "version": "2.1",
                "provides": [{"id": "docker.compose", "version": "2.24"}]}"#,
        );
        system.write(
            "usr/lib/system-runtime/components/app.json",
            r#"{"id": "app", "owner": "rugix.apps.app", "version": "1.0",
                "requires": [{"id": "docker.compose", "version": ">=2.20,<3"}]}"#,
        );
        system.write(
            "usr/lib/system-runtime/health/nodes/apps.json",
            r#"{"id": "apps", "rule": {"type": "Aggregate"}}"#,
        );
        system.write(
            "usr/lib/system-runtime/health/nodes/app.json",
            r#"{"id": "app", "partOf": [{"node": "apps"}]}"#,
        );
        system.write(
            "usr/lib/system-runtime/health/nodes/db.json",
            r#"{"id": "db", "partOf": [{"node": "apps"}]}"#,
        );
        let context = HealthContext::current().unwrap();
        system.write(
            "run/system-runtime/health/status/app.json",
            &format!(
                r#"{{"id": "app", "usable": "Yes", "healthy": "No", "bootId": "{}",
                    "observedAtMs": {}, "maxAgeMs": 3600000, "message": "queue backlog"}}"#,
                context.boot_id, context.now_ms
            ),
        );
        system
    }

    fn write(&self, path: &str, content: &str) {
        let path = self.root.path().join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    fn run(&self, args: &[&str]) -> Output {
        run(&[&["--root", self.root.path().to_str().unwrap()], args].concat())
    }
}

/// Lists the declarations as a table and as JSON.
#[test]
fn components_list() {
    let system = System::new();
    let output = system.run(&["components", "list"]);
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8(output.stdout).unwrap();
    let app = stdout
        .lines()
        .find(|line| line.starts_with("app "))
        .unwrap();
    assert!(app.contains("rugix.apps.app") && app.contains("docker.compose >=2.20,<3"));
    let output = system.run(&["components", "list", "--json"]);
    let components: Vec<ComponentDeclaration> = decode(&output);
    assert_eq!(components.len(), 2);
}

/// Accepts the current composition and rejects an update that breaks a requirement.
#[test]
fn components_check_with_update() {
    let system = System::new();
    let output = system.run(&["components", "check"]);
    assert_eq!(output.status.code(), Some(0));
    let update = System::new();
    update.write(
        "usr/lib/system-runtime/components/os.json",
        r#"{"id": "os", "owner": "rugix.system", "version": "3.0",
            "provides": [{"id": "docker.compose", "version": "3.1"}]}"#,
    );
    let update_root = update.root.path().to_str().unwrap();
    let args = [
        "components",
        "check",
        "--update",
        update_root,
        "--owner",
        "rugix.system",
    ];
    let output = system.run(&args);
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("app requires docker.compose >=2.20,<3, which no component provides"));
    let report: CompatibilityReport = decode(&system.run(&[&args[..], &["--json"]].concat()));
    assert!(!report.consistent);
}

/// Reports an invalid record on standard error and the composition as inconsistent.
#[test]
fn components_check_skipped_record() {
    let system = System::new();
    system.write("usr/lib/system-runtime/components/broken.json", "{");
    let output = system.run(&["components", "check"]);
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.starts_with("error: ") && stderr.contains("broken.json"));
}

/// Reports yes, no, and unknown through the exit code, and an unknown node as a failure.
#[test]
fn health_check_exit_codes() {
    let system = System::new();
    let code = |args: &[&str]| {
        system
            .run(&[&["health", "check"], args].concat())
            .status
            .code()
    };
    assert_eq!(code(&["app", "--usable"]), Some(0));
    assert_eq!(code(&["app"]), Some(1));
    assert_eq!(code(&["db"]), Some(2));
    assert_eq!(code(&["missing"]), Some(3));
}

/// Shows the health tree as text, and the results of one subtree as JSON.
#[test]
fn health_show() {
    let system = System::new();
    let output = system.run(&["health", "show"]);
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 4);
    assert!(lines[2].starts_with("yes") && lines[2].ends_with("├── app: queue backlog"));
    assert!(lines[3].ends_with("└── db (missing)"));
    let report: HealthReport = decode(&system.run(&["health", "show", "app", "--json"]));
    assert_eq!(report.results.len(), 1);
}

/// Exits with 3 on invalid arguments, so that they cannot be mistaken for an answer.
#[test]
fn invalid_arguments() {
    assert_eq!(run(&["health", "check"]).status.code(), Some(3));
    let missing_root = ["--root", "/nonexistent", "components", "check"];
    assert_eq!(run(&missing_root).status.code(), Some(3));
    assert_eq!(run(&["--version"]).status.code(), Some(0));
}

fn run(args: &[&str]) -> Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_system-runtime"))
        .args(args)
        .output()
        .unwrap()
}

fn decode<T: sidex::Decode>(output: &Output) -> T {
    sidex::formats::json::from_slice(&output.stdout).unwrap()
}
