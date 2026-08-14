use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process;

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

    let (action, scenario) = match cli.command {
        Commands::Up { scenario } => ("UP", scenario),

        Commands::Down { scenario } => ("DOWN", scenario),

        Commands::List => {
            list_scenarios();
            return;
        }
    };

    let path = format!("labs/{}", scenario);
    if !Path::new(&path).is_dir() {
        eprintln!("Error: Scenario '{}' not found.", scenario);
        process::exit(1);
    }

    println!("Action: {} | Scenario: {}", action, scenario);
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
            process::exit(1);
        }
    }
}
