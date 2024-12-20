#![feature(linked_list_cursors)]
extern crate core;
extern crate pest;

use clap::{Parser, ValueEnum};
use spinners::{Spinner, Spinners};
use std::fmt::Display;
use std::time::{Duration, Instant};

mod solver;
use solver::{
    solver_2015_01, solver_2015_02, solver_2015_03, solver_2015_04, solver_2015_05, solver_2015_06,
    solver_2015_07, solver_2022_01, solver_2022_02, solver_2022_03, solver_2022_04, Solver,
    Solver2015_08, Solver2022_05, Solver2022_06, Solver2022_07, Solver2022_08, Solver2022_09,
    Solver2022_10, Solver2022_11, Solver2022_12, Solver2022_13, Solver2022_14, Solver2022_15,
    Solver2022_16, Solver2022_17, Solver2022_18, Solver2022_20, Solver2022_21, Solver2022_22,
    Solver2024_01, Solver2024_02, Solver2024_03, Solver2024_04, Solver2024_05, Solver2024_06,
    Solver2024_07, Solver2024_08, Solver2024_09, Solver2024_10, Solver2024_11, Solver2024_12,
    Solver2024_13, Solver2024_14, Solver2024_15, Solver2024_16, Solver2024_17, Solver2024_18,
    Solver2024_19,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Year {
    #[value(name = "2015")]
    Year2015,
    #[value(name = "2022")]
    Year2022,
    #[value(name = "2024")]
    Year2024,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Day {
    #[value(name = "1")]
    Day1,
    #[value(name = "2")]
    Day2,
    #[value(name = "3")]
    Day3,
    #[value(name = "4")]
    Day4,
    #[value(name = "5")]
    Day5,
    #[value(name = "6")]
    Day6,
    #[value(name = "7")]
    Day7,
    #[value(name = "8")]
    Day8,
    #[value(name = "9")]
    Day9,
    #[value(name = "10")]
    Day10,
    #[value(name = "11")]
    Day11,
    #[value(name = "12")]
    Day12,
    #[value(name = "13")]
    Day13,
    #[value(name = "14")]
    Day14,
    #[value(name = "15")]
    Day15,
    #[value(name = "16")]
    Day16,
    #[value(name = "17")]
    Day17,
    #[value(name = "18")]
    Day18,
    #[value(name = "19")]
    Day19,
    #[value(name = "20")]
    Day20,
    #[value(name = "21")]
    Day21,
    #[value(name = "22")]
    Day22,
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_enum)]
    year: Year,

    #[arg(value_enum)]
    day: Day,
}

fn print_answers<T1: Display, T2: Display>((first_answer, second_answer): (T1, T2)) {
    println!("Answer for the first part:");
    println!("--------------------------");
    println!("{}", first_answer);
    println!();
    println!("Answer for the second part:");
    println!("---------------------------");
    println!("{}", second_answer);
}

fn solve<T1: Display, T2: Display>(solver: Box<dyn Solver<T1, T2>>) {
    let mut sp = Spinner::new(Spinners::Dots, "Solving the first part...".to_string());
    let start = Instant::now();
    let first_answer = solver.solve_first_part();
    let mut count = 1;
    while start.elapsed() < Duration::from_secs(1) {
        solver.solve_first_part();
        count += 1;
    }
    let elapsed = start.elapsed() / count;

    sp.stop_with_symbol("✅");
    println!();
    println!("Answer for the first part:");
    println!("--------------------------");
    println!("{first_answer}");
    println!("Time elapsed: {elapsed:?}");

    let mut sp = Spinner::new(Spinners::Dots, "Solving the second part...".to_string());
    let start = Instant::now();
    let second_answer = solver.solve_second_part();
    let mut count = 1;
    while start.elapsed() < Duration::from_secs(1) {
        solver.solve_second_part();
        count += 1;
    }
    let elapsed = start.elapsed() / count;

    sp.stop_with_symbol("✅");
    println!();
    println!("Answer for the second part:");
    println!("---------------------------");
    println!("{second_answer}");
    println!("Time elapsed: {elapsed:?}");
}

fn main() {
    let args = Cli::parse();
    match (args.year, args.day) {
        (Year::Year2015, Day::Day1) => print_answers(solver_2015_01::solve()),
        (Year::Year2015, Day::Day2) => print_answers(solver_2015_02::solve()),
        (Year::Year2015, Day::Day3) => print_answers(solver_2015_03::solve()),
        (Year::Year2015, Day::Day4) => print_answers(solver_2015_04::solve()),
        (Year::Year2015, Day::Day5) => print_answers(solver_2015_05::solve()),
        (Year::Year2015, Day::Day6) => print_answers(solver_2015_06::solve()),
        (Year::Year2015, Day::Day7) => print_answers(solver_2015_07::solve()),
        (Year::Year2015, Day::Day8) => solve(Box::new(Solver2015_08::default())),
        (Year::Year2022, Day::Day1) => print_answers(solver_2022_01::solve()),
        (Year::Year2022, Day::Day2) => print_answers(solver_2022_02::solve()),
        (Year::Year2022, Day::Day3) => print_answers(solver_2022_03::solve()),
        (Year::Year2022, Day::Day4) => print_answers(solver_2022_04::solve()),
        (Year::Year2022, Day::Day5) => solve(Box::new(Solver2022_05::default())),
        (Year::Year2022, Day::Day6) => solve(Box::new(Solver2022_06::default())),
        (Year::Year2022, Day::Day7) => solve(Box::new(Solver2022_07::default())),
        (Year::Year2022, Day::Day8) => solve(Box::new(Solver2022_08::default())),
        (Year::Year2022, Day::Day9) => solve(Box::new(Solver2022_09::default())),
        (Year::Year2022, Day::Day10) => solve(Box::new(Solver2022_10::default())),
        (Year::Year2022, Day::Day11) => solve(Box::new(Solver2022_11::default())),
        (Year::Year2022, Day::Day12) => solve(Box::new(Solver2022_12::default())),
        (Year::Year2022, Day::Day13) => solve(Box::new(Solver2022_13::default())),
        (Year::Year2022, Day::Day14) => solve(Box::new(Solver2022_14::default())),
        (Year::Year2022, Day::Day15) => solve(Box::new(Solver2022_15::default())),
        (Year::Year2022, Day::Day16) => solve(Box::new(Solver2022_16::default())),
        (Year::Year2022, Day::Day17) => solve(Box::new(Solver2022_17::default())),
        (Year::Year2022, Day::Day18) => solve(Box::new(Solver2022_18::default())),
        (Year::Year2022, Day::Day20) => solve(Box::new(Solver2022_20::default())),
        (Year::Year2022, Day::Day21) => solve(Box::new(Solver2022_21::default())),
        (Year::Year2022, Day::Day22) => solve(Box::new(Solver2022_22::default())),
        (Year::Year2024, Day::Day1) => solve(Box::new(Solver2024_01::default())),
        (Year::Year2024, Day::Day2) => solve(Box::new(Solver2024_02::default())),
        (Year::Year2024, Day::Day3) => solve(Box::new(Solver2024_03::default())),
        (Year::Year2024, Day::Day4) => solve(Box::new(Solver2024_04::default())),
        (Year::Year2024, Day::Day5) => solve(Box::new(Solver2024_05::default())),
        (Year::Year2024, Day::Day6) => solve(Box::new(Solver2024_06::default())),
        (Year::Year2024, Day::Day7) => solve(Box::new(Solver2024_07::default())),
        (Year::Year2024, Day::Day8) => solve(Box::new(Solver2024_08::default())),
        (Year::Year2024, Day::Day9) => solve(Box::new(Solver2024_09::default())),
        (Year::Year2024, Day::Day10) => solve(Box::new(Solver2024_10::default())),
        (Year::Year2024, Day::Day11) => solve(Box::new(Solver2024_11::default())),
        (Year::Year2024, Day::Day12) => solve(Box::new(Solver2024_12::default())),
        (Year::Year2024, Day::Day13) => solve(Box::new(Solver2024_13::default())),
        (Year::Year2024, Day::Day14) => solve(Box::new(Solver2024_14::default())),
        (Year::Year2024, Day::Day15) => solve(Box::new(Solver2024_15::default())),
        (Year::Year2024, Day::Day16) => solve(Box::new(Solver2024_16::default())),
        (Year::Year2024, Day::Day17) => solve(Box::new(Solver2024_17::default())),
        (Year::Year2024, Day::Day18) => solve(Box::new(Solver2024_18::default())),
        (Year::Year2024, Day::Day19) => solve(Box::new(Solver2024_19::default())),
        _ => panic!("Puzzle is not solved yet!"),
    }
}
