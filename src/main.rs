use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25, // ... 날짜별 추가
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    println!("Advent of Code 2024 !");
    let cli = Cli::parse();

    match cli.command {
        Commands::Day01 => day01::solve(),
        Commands::Day02 => day02::solve(),
        Commands::Day03 => day03::solve(),
        Commands::Day04 => day04::solve(),
        Commands::Day05 => day05::solve(),
        Commands::Day06 => day06::solve(),
        Commands::Day07 => day07::solve(),
        Commands::Day08 => day08::solve(),
        Commands::Day09 => day09::solve(),
        Commands::Day10 => day10::solve(),
        Commands::Day11 => day11::solve(),
        Commands::Day12 => day12::solve(),
        Commands::Day13 => day13::solve(),
        Commands::Day14 => day14::solve(),
        Commands::Day15 => day15::solve(),
        Commands::Day16 => day16::solve(),
        Commands::Day17 => day17::solve(),
        Commands::Day18 => day18::solve(),
        Commands::Day19 => day19::solve(),
        Commands::Day20 => day20::solve(),
        Commands::Day21 => day21::solve(),
        Commands::Day22 => day22::solve(),
        Commands::Day23 => day23::solve(),
        Commands::Day24 => day24::solve(),
        Commands::Day25 => day25::solve(),
    }
}
