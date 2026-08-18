use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, exit};

#[derive(Parser)]
#[command(name = "labnet")]
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

    match cli.command {
        Commands::Up { scenario } => {
            let compose_path = validate_scenario(&scenario);
            run_docker_compose(&compose_path, "up");

            let mission_path = format!("labs/{}/MISSION.md", scenario);
            match fs::read_to_string(&mission_path) {
                Ok(content) => println!("\n{}\n", content),
                Err(_) => println!("Note: No MISSION.md found for this scenario."),
            }
        }

        Commands::Down { scenario } => {
            let compose_path = validate_scenario(&scenario);
            run_docker_compose(&compose_path, "down");
        }

        Commands::List => {
            list_scenarios();
        }

        Commands::Hint { scenario } => {
            let _ = validate_scenario(&scenario);
            let hint_path = format!("labs/{}/HINTS.md", scenario);
            match fs::read_to_string(&hint_path) {
                Ok(content) => println!("{}", content),
                Err(_) => {
                    eprintln!("Error: No HINTS.md found for scenario '{}'.", scenario);
                    exit(1);
                }
            }
        }

        Commands::Solution { scenario } => {
            let _ = validate_scenario(&scenario);
            let solution_path = format!("labs/{}/SOLUTION.md", scenario);
            match fs::read_to_string(&solution_path) {
                Ok(content) => println!("{}", content),
                Err(_) => {
                    eprintln!("Error: No SOLUTION.md found for scenario '{}'.", scenario);
                    exit(1);
                }
            }
        }

        Commands::Complete { scenario, undo } => {
            let _ = validate_scenario(&scenario);
            let mut completed = read_completed();

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
                if completed.contains(&scenario) {
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
            let running = get_running_labs();

            match fs::read_dir("labs") {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        if entry.file_type().is_ok_and(|t| t.is_dir()) {
                            let name = entry.file_name().to_string_lossy().to_string();
                            let is_running = running.contains(&name);
                            let is_completed = completed.contains(&name);

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
                }
                Err(_) => {
                    eprintln!("Error: 'labs/' directory not found.");
                    exit(1);
                }
            }
        }
    };
}

fn validate_scenario(scenario: &str) -> String {
    let dir_path = format!("labs/{}", scenario);
    if !Path::new(&dir_path).is_dir() {
        eprintln!("Error: Scenario '{}' not found.", scenario);
        exit(1);
    }

    let compose_path = format!("{}/docker-compose.yml", dir_path);
    if !Path::new(&compose_path).is_file() {
        eprintln!(
            "Error: docker-compose.yml not found in scenario '{}'.",
            scenario
        );
        exit(1);
    }
    compose_path
}

fn run_docker_compose(compose_path: &str, action: &str) {
    let args = match action {
        "up" => vec!["compose", "-f", compose_path, "up", "-d"],
        "down" => vec!["compose", "-f", compose_path, "down"],
        _ => {
            eprintln!("Error: Invalid action '{}'.", action);
            exit(1);
        }
    };

    match Command::new("docker").args(&args).status() {
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

fn list_scenarios() {
    let completed = read_completed();
    if completed.is_empty() {
        println!("Note: No labs completed yet.");
    }

    match fs::read_dir("labs") {
        Ok(entries) => {
            for entry in entries.flatten() {
                if entry.file_type().is_ok_and(|t| t.is_dir()) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if completed.contains(&name) {
                        println!("[COMPLETED] {}", name);
                    } else {
                        println!("{}", name);
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("Error: 'labs/' directory not found.");
            exit(1);
        }
    }
}

fn get_running_labs() -> Vec<String> {
    let mut running = Vec::new();
    if let Ok(entries) = fs::read_dir("labs") {
        for entry in entries.flatten() {
            if entry.file_type().is_ok_and(|t| t.is_dir()) {
                let name = entry.file_name().to_string_lossy().to_string();
                let compose_path = format!("labs/{}/docker-compose.yml", name);

                match Command::new("docker")
                    .args(["compose", "-f", &compose_path, "ps", "--quiet"])
                    .output()
                {
                    Ok(output) if output.status.success() && !output.stdout.is_empty() => {
                        running.push(name);
                    }
                    _ => continue,
                }
            }
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
        Err(_) => Vec::new(),
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
