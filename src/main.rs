use clap::{Parser, Subcommand};

/// A simple and efficient tool for managing and running scripts
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available scripts
    List,
}

fn main() {
    let _cli = Cli::parse();
}
