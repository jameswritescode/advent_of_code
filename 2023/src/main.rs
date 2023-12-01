use clap::{Parser, Subcommand};
use std::fs;

mod day1;

#[derive(Parser)]
struct Cli {
    #[arg(required = true)]
    input: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Day1,
}

fn main() {
    let cli = Cli::parse();
    let input = fs::read_to_string(cli.input).unwrap_or_default();

    match &cli.command {
        Some(Commands::Day1) => day1::run(input),

        None => {}
    }
}
