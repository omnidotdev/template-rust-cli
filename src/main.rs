use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "{{binary-name}}")]
#[command(about = "{{description}}")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Example command
    Hello {
        /// Name to greet
        #[arg(short, long, default_value = "world")]
        name: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { name } => {
            println!("Hello, {name}!");
            ExitCode::SUCCESS
        }
    }
}
