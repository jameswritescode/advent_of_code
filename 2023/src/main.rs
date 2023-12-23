use clap::{Parser, Subcommand};
use std::fs;

mod day1;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day14,
    Day15,
}

fn main() {
    let cli = Cli::parse();
    let input = fs::read_to_string(cli.input).unwrap_or_default();

    match &cli.command {
        Some(Commands::Day1) => day1::run(input),
        Some(Commands::Day2) => day2::run(input),
        Some(Commands::Day3) => day3::run(input),
        Some(Commands::Day4) => day4::run(input),
        Some(Commands::Day5) => day5::run(input),
        Some(Commands::Day6) => day6::run(input),
        Some(Commands::Day7) => day7::run(input),
        Some(Commands::Day8) => day8::run(input),
        Some(Commands::Day9) => day9::run(input),
        Some(Commands::Day14) => day14::run(input),
        Some(Commands::Day15) => day15::run(input),
        None => {}
    }
}
