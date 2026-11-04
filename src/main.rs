use clap::{Parser, Subcommand};
use std::fs;
use std::io::ErrorKind;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, exit};

#[derive(Parser)]
#[command(name = "labnet")]
#[command(version)]
#[command(about = "A local cybersecurity lab generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start up a lab scenario
    Up { scenario: String },
    /// Stop and remove a lab scenario
    Down { scenario: String },
    /// List available lab scenarios
    List,
    /// Print hints for a lab scenario
    Hint { scenario: String },
    /// Print the solution for a lab scenario
    Solution { scenario: String },
    /// Mark a lab scenario as completed or undo completion
    Complete {
        scenario: String,
        #[arg(long)]
        undo: bool,
    },
    /// Show running and completed status of all scenarios
    Status,
}

fn main() {
    let cli = Cli::parse();
    let labs = labs_dir();

    match cli.command {
        Commands::Up { scenario } => {
            let compose_path = validate_scenario(&labs, &scenario);
            run_docker_compose(&compose_path, "up");

            let mission_path = labs.join(&scenario).join("MISSION.md");
            match fs::read_to_string(&mission_path) {
                Ok(content) => println!("\n{}\n", content),
                Err(_) => println!("Note: No MISSION.md found for this scenario."),
            }
        }

        Commands::Down { scenario } => {
            let compose_path = validate_scenario(&labs, &scenario);
            run_docker_compose(&compose_path, "down");
        }

        Commands::List => {
            list_scenarios(&labs);
        }

        Commands::Hint { scenario } => {
            validate_scenario(&labs, &scenario);
            let hint_path = labs.join(&scenario).join("HINTS.md");
            match fs::read_to_string(&hint_path) {
                Ok(content) => println!("{}", content),
                Err(_) => {
                    eprintln!("Error: No HINTS.md found for scenario '{}'.", scenario);
                    exit(1);
                }
            }
        }

        Commands::Solution { scenario } => {
            validate_scenario(&labs, &scenario);
            let solution_path = labs.join(&scenario).join("SOLUTION.md");
            match fs::read_to_string(&solution_path) {
                Ok(content) => println!("{}", content),
                Err(_) => {
                    eprintln!("Error: No SOLUTION.md found for scenario '{}'.", scenario);
                    exit(1);
                }
            }
        }

        Commands::Complete { scenario, undo } => {
            if undo {
                validate_scenario_name(&scenario);
            } else {
                validate_scenario(&labs, &scenario);
            }
            let mut completed = read_completed();
            let scenario = if undo {
                canonical_scenario_name(&completed, &scenario)
                    .or_else(|| canonical_scenario_name(&scenario_names(&labs), &scenario))
                    .unwrap_or(scenario)
            } else {
                canonical_scenario_name(&scenario_names(&labs), &scenario).unwrap_or(scenario)
            };

            if undo {
                let initial_len = completed.len();
                completed.retain(|s| s != &scenario);
                if completed.len() == initial_len {
                    eprintln!("Error: Scenario '{}' is not marked as completed.", scenario);
                    exit(1);
                }
                write_completed(&completed);
                println!("Scenario '{}' set as incomplete.", scenario);
            } else {
                if canonical_scenario_name(&completed, &scenario).is_some() {
                    println!(
                        "Note: Scenario '{}' is already marked as completed.",
                        scenario
                    );
                } else {
                    completed.push(scenario.clone());
                    write_completed(&completed);
                    println!("Scenario '{}' marked as completed.", scenario);
                }
            }
        }

        Commands::Status => {
            let completed = read_completed();
            let scenarios = scenario_names(&labs);
            let running = get_running_labs(&labs, &scenarios);

            for name in scenarios {
                let is_running = running.contains(&name);
                let is_completed = canonical_scenario_name(&completed, &name).is_some();

                let mut status = String::new();
                if is_running {
                    status.push_str("[RUNNING] ");
                } else {
                    status.push_str("[STOPPED] ");
                }

                if is_completed {
                    status.push_str("[COMPLETED] ");
                }

                println!("{}{}", status, name);
            }
        }
    };
}

fn is_valid_scenario_name(name: &str) -> bool {
    let mut components = Path::new(name).components();

    !name.contains('\\')
        && matches!(components.next(), Some(Component::Normal(_)))
        && components.next().is_none()
}

fn validate_scenario_name(scenario: &str) {
    if !is_valid_scenario_name(scenario) {
        eprintln!("Error: Invalid scenario name '{}'.", scenario);
        exit(1);
    }
}

fn canonical_scenario_name(names: &[String], input: &str) -> Option<String> {
    names
        .iter()
        .find(|name| name.as_str() == input)
        .or_else(|| {
            if cfg!(windows) {
                names.iter().find(|name| name.eq_ignore_ascii_case(input))
            } else {
                None
            }
        })
        .cloned()
}

fn validate_scenario(labs: &Path, scenario: &str) -> PathBuf {
    validate_scenario_name(scenario);

    let dir_path = labs.join(scenario);
    if !dir_path.is_dir() {
        eprintln!("Error: Scenario '{}' not found.", scenario);
        exit(1);
    }

    let compose_path = dir_path.join("docker-compose.yml");
    if !compose_path.is_file() {
        eprintln!(
            "Error: docker-compose.yml not found in scenario '{}'.",
            scenario
        );
        exit(1);
    }
    compose_path
}

fn run_docker_compose(compose_path: &Path, action: &str) {
    let args: &[&str] = match action {
        "up" => &["up", "-d"],
        "down" => &["down"],
        _ => {
            eprintln!("Error: Invalid action '{}'.", action);
            exit(1);
        }
    };

    match Command::new("docker")
        .args(["compose", "-f"])
        .arg(compose_path)
        .args(args)
        .status()
    {
        Ok(exit_status) => {
            if !exit_status.success() {
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to execute docker. {}", e);
            exit(1);
        }
    }
}

fn resolve_labs_dir(exe_dir: Option<&Path>, cwd: &Path) -> Option<PathBuf> {
    if let Some(dir) = exe_dir {
        let candidate = dir.join("labs");
        if candidate.is_dir() {
            return Some(candidate);
        }
    }

    let candidate = cwd.join("labs");
    candidate.is_dir().then_some(candidate)
}

fn labs_dir() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf));

    match resolve_labs_dir(exe_dir.as_deref(), &cwd) {
        Some(path) => path,
        None => {
            eprintln!("Error: Could not find the 'labs' directory.");
            exit(1);
        }
    }
}

fn is_scenario_dir(path: &Path) -> bool {
    path.is_dir() && path.join("docker-compose.yml").is_file()
}

fn scenario_names(labs: &Path) -> Vec<String> {
    let entries = match fs::read_dir(labs) {
        Ok(entries) => entries,
        Err(_) => {
            eprintln!("Error: Could not read labs directory '{}'.", labs.display());
            exit(1);
        }
    };

    let mut names: Vec<String> = entries
        .flatten()
        .filter(|entry| is_scenario_dir(&entry.path()))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

fn list_scenarios(labs: &Path) {
    let completed = read_completed();
    if completed.is_empty() {
        println!("Note: No labs completed yet.");
    }

    for name in scenario_names(labs) {
        if canonical_scenario_name(&completed, &name).is_some() {
            println!("[COMPLETED] {}", name);
        } else {
            println!("{}", name);
        }
    }
}

fn get_running_labs(labs: &Path, scenarios: &[String]) -> Vec<String> {
    let mut running = Vec::new();
    for name in scenarios {
        let compose_path = labs.join(name).join("docker-compose.yml");

        let output = Command::new("docker")
            .args(["compose", "-f"])
            .arg(&compose_path)
            .args(["ps", "--quiet"])
            .output()
            .unwrap_or_else(|e| {
                eprintln!("Error: Failed to execute docker. {}", e);
                exit(1);
            });

        if !output.status.success() {
            eprintln!(
                "Error: Failed to query status of scenario '{}'. {}",
                name,
                String::from_utf8_lossy(&output.stderr).trim()
            );
            exit(1);
        }

        if !output.stdout.is_empty() {
            running.push(name.clone());
        }
    }
    running
}

fn get_state_path() -> PathBuf {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => {
            eprintln!("Error: Could not determine home directory.");
            exit(1);
        }
    };
    home.join(".labnet/state.json")
}

fn read_completed() -> Vec<String> {
    let path = get_state_path();
    match fs::read_to_string(&path) {
        Ok(contents) => match serde_json::from_str(&contents) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error: Failed to parse state file. {}", e);
                exit(1);
            }
        },
        Err(e) if e.kind() == ErrorKind::NotFound => Vec::new(),
        Err(e) => {
            eprintln!("Error: Failed to read state file. {}", e);
            exit(1);
        }
    }
}

fn write_completed(completed: &[String]) {
    let path = get_state_path();
    if let Some(parent) = path.parent()
        && let Err(e) = fs::create_dir_all(parent)
    {
        eprintln!("Error: Failed to create state directory. {}", e);
        exit(1);
    }

    let json = match serde_json::to_string_pretty(completed) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Error: Failed to serialize state. {}", e);
            exit(1);
        }
    };

    if let Err(e) = fs::write(&path, json) {
        eprintln!("Error: Failed to write state file. {}", e);
        exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_directory_is_not_a_scenario() {
        assert!(is_scenario_dir(Path::new("labs/dvwa")));
        assert!(!is_scenario_dir(Path::new("labs/template")));
    }

    #[test]
    fn canonical_scenario_name_prefers_exact_match() {
        let names = vec!["alpha".to_string(), "Alpha".to_string()];

        assert_eq!(
            canonical_scenario_name(&names, "Alpha"),
            Some("Alpha".to_string())
        );
        assert_eq!(
            canonical_scenario_name(&names, "alpha"),
            Some("alpha".to_string())
        );
        assert_eq!(canonical_scenario_name(&names, "missing"), None);
    }

    #[cfg(windows)]
    #[test]
    fn canonical_scenario_name_matches_case_insensitively_on_windows() {
        let names = vec!["alpha".to_string()];

        assert_eq!(
            canonical_scenario_name(&names, "Alpha"),
            Some("alpha".to_string())
        );
        assert_eq!(
            canonical_scenario_name(&names, "ALPHA"),
            Some("alpha".to_string())
        );
    }

    #[test]
    fn executable_adjacent_labs_directory_takes_precedence() {
        let labs = resolve_labs_dir(Some(Path::new(".")), Path::new("missing"));
        assert_eq!(labs, Some(Path::new(".").join("labs")));
    }

    #[test]
    fn working_directory_is_used_when_no_labs_beside_executable() {
        let labs = resolve_labs_dir(Some(Path::new("missing")), Path::new("."));
        assert_eq!(labs, Some(Path::new(".").join("labs")));
    }

    #[test]
    fn missing_labs_directory_returns_none() {
        assert_eq!(resolve_labs_dir(None, Path::new("missing")), None);
    }

    #[test]
    fn scenario_names_cannot_escape_labs_directory() {
        assert!(is_valid_scenario_name("dvwa"));
        assert!(!is_valid_scenario_name(""));
        assert!(!is_valid_scenario_name("."));
        assert!(!is_valid_scenario_name(".."));
        assert!(!is_valid_scenario_name("../dvwa"));
        assert!(!is_valid_scenario_name("a/b"));
        assert!(!is_valid_scenario_name("a\\b"));
    }
}
