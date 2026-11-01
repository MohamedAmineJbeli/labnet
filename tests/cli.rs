use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;

struct Fixture {
    root: PathBuf,
    home: PathBuf,
    elsewhere: PathBuf,
}

impl Fixture {
    fn new(name: &str) -> Self {
        let root = env::temp_dir().join(format!("labnet-cli-{}-{}", std::process::id(), name));
        let _ = fs::remove_dir_all(&root);

        let home = root.join("home");
        let elsewhere = root.join("elsewhere");
        fs::create_dir_all(&home).unwrap();
        fs::create_dir_all(&elsewhere).unwrap();

        let zeta = root.join("labs/zeta");
        fs::create_dir_all(&zeta).unwrap();
        fs::write(zeta.join("docker-compose.yml"), "").unwrap();

        let lab = root.join("labs/alpha");
        fs::create_dir_all(&lab).unwrap();
        fs::write(lab.join("docker-compose.yml"), "").unwrap();
        fs::write(lab.join("MISSION.md"), "Alpha mission\n").unwrap();
        fs::write(lab.join("HINTS.md"), "Alpha hint\n").unwrap();
        fs::write(lab.join("SOLUTION.md"), "Alpha solution\n").unwrap();

        let template = root.join("labs/template");
        fs::create_dir_all(&template).unwrap();
        fs::write(template.join("README.md"), "Template\n").unwrap();

        let binary = env!("CARGO_BIN_EXE_labnet");
        fs::copy(
            binary,
            root.join(format!("labnet{}", env::consts::EXE_SUFFIX)),
        )
        .unwrap();

        Fixture {
            root,
            home,
            elsewhere,
        }
    }

    fn run(&self, args: &[&str]) -> Output {
        let binary = self.root.join(format!("labnet{}", env::consts::EXE_SUFFIX));
        let mut attempts = 0;

        loop {
            let result = Command::new(&binary)
                .args(args)
                .current_dir(&self.elsewhere)
                .env("HOME", &self.home)
                .output();

            match result {
                Ok(output) => return output,
                // Exec can briefly return ETXTBSY after a binary is copied.
                Err(e) if e.kind() == ErrorKind::ExecutableFileBusy && attempts < 20 => {
                    attempts += 1;
                    thread::sleep(Duration::from_millis(5));
                }
                Err(e) => panic!("failed to run labnet: {e}"),
            }
        }
    }

    fn state_path(&self) -> PathBuf {
        self.home.join(".labnet/state.json")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn assert_success(output: &Output) {
    assert!(output.status.success(), "stderr: {}", stderr(output));
}

#[test]
fn list_excludes_directories_without_compose_file() {
    let fixture = Fixture::new("list");

    let output = fixture.run(&["list"]);
    assert_success(&output);
    let out = stdout(&output);
    assert!(out.contains("alpha"), "stdout: {out}");
    assert!(!out.contains("template"), "stdout: {out}");
}

#[test]
fn list_is_sorted_alphabetically() {
    let fixture = Fixture::new("list-order");

    let output = fixture.run(&["list"]);
    assert_success(&output);
    let out = stdout(&output);
    let alpha = out.find("alpha").unwrap();
    let zeta = out.find("zeta").unwrap();
    assert!(alpha < zeta, "stdout: {out}");
}

#[cfg(unix)]
#[test]
fn complete_persists_state() {
    let fixture = Fixture::new("complete");

    let output = fixture.run(&["complete", "alpha"]);
    assert_success(&output);
    assert!(stdout(&output).contains("marked as completed"));

    let state = fs::read_to_string(fixture.state_path()).unwrap();
    assert!(state.contains("\"alpha\""), "state: {state}");

    let output = fixture.run(&["list"]);
    assert_success(&output);
    assert!(stdout(&output).contains("[COMPLETED] alpha"));
}

#[test]
fn hint_prints_hints_file() {
    let fixture = Fixture::new("hint");

    let output = fixture.run(&["hint", "alpha"]);
    assert_success(&output);
    assert!(stdout(&output).contains("Alpha hint"));
}

#[test]
fn hint_rejects_missing_scenario() {
    let fixture = Fixture::new("hint-missing");

    let output = fixture.run(&["hint", "missing"]);
    assert!(!output.status.success());
    assert!(stderr(&output).contains("Scenario 'missing' not found."));
}

#[test]
fn solution_prints_solution_file() {
    let fixture = Fixture::new("solution");

    let output = fixture.run(&["solution", "alpha"]);
    assert_success(&output);
    assert!(stdout(&output).contains("Alpha solution"));
}

#[test]
fn rejects_path_traversal_names() {
    let fixture = Fixture::new("traversal");

    let output = fixture.run(&["up", "../alpha"]);
    assert!(!output.status.success());
    assert!(stderr(&output).contains("Invalid scenario name '../alpha'."));
}

#[test]
fn version_matches_package_version() {
    let fixture = Fixture::new("version");

    let output = fixture.run(&["--version"]);
    assert_success(&output);
    assert_eq!(
        stdout(&output),
        format!("labnet {}\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn runs_from_archive_layout() {
    let fixture = Fixture::new("archive-layout");
    assert!(!fixture.elsewhere.join("labs").exists());

    let output = fixture.run(&["list"]);
    assert_success(&output);
    assert!(stdout(&output).contains("alpha"));
}
