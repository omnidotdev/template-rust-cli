use std::process::ExitCode;

use clap::{CommandFactory, Parser, Subcommand};

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
    /// Print a shell completion script (bash, zsh, fish, ...)
    Completions {
        /// Shell to generate completions for
        shell: clap_complete::Shell,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { name } => {
            println!("Hello, {name}!");
            ExitCode::SUCCESS
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
            ExitCode::SUCCESS
        }
    }
}
