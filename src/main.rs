use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
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
    match fs::read_dir("labs") {
        Ok(entries) => {
            for entry in entries.flatten() {
                if entry.file_type().is_ok_and(|t| t.is_dir()) {
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
        }
        Err(_) => {
            eprintln!("Error: 'labs/' directory not found.");
            exit(1);
        }
    }
}
