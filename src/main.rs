use clap::{Parser, Subcommand};

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
            println!("Starting scenario: {}", scenario);
        }

        Commands::Down { scenario } => {
            println!("Stopping scenario: {}", scenario);
        }

        Commands::List => {
            println!("Listing scenarios");
        }
    }
}
