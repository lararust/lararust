pub mod serve;

use clap::{Parser, Subcommand};
use serve::serve_command;

#[derive(Parser)]
#[command(name = "LaraRust", version, about = "Framework inspired in Laravel, made in Rust.")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Serve,
}

pub async fn run_cli() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Serve) | None => serve_command().await,
    }
}
