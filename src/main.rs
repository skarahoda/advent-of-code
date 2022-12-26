extern crate core;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fmt::Display;
use clap::{Parser, ValueEnum};
mod solver;
use solver::{
    solver_2015_01,
    solver_2015_02,
    solver_2015_03,
    solver_2015_04,
    solver_2015_05,
    solver_2015_06,
    solver_2015_07,
    solver_2022_01,
    solver_2022_02,
    solver_2022_03,
    solver_2022_04,
    solver_2022_05,
    solver_2022_06,
    solver_2022_07,
    solver_2022_08,
    solver_2022_09,
    solver_2022_10,
    solver_2022_11,
    solver_2022_12,
    solver_2022_13,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Year {
    #[value(name="2015")]
    Year2015,
    #[value(name="2022")]
    Year2022,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Day {
    #[value(name="1")]
    Day1,
    #[value(name="2")]
    Day2,
    #[value(name="3")]
    Day3,
    #[value(name="4")]
    Day4,
    #[value(name="5")]
    Day5,
    #[value(name="6")]
    Day6,
    #[value(name="7")]
    Day7,
    #[value(name="8")]
    Day8,
    #[value(name="9")]
    Day9,
    #[value(name="10")]
    Day10,
    #[value(name="11")]
    Day11,
    #[value(name="12")]
    Day12,
    #[value(name="13")]
    Day13,
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
    println!("");
    println!("Answer for the second part:");
    println!("---------------------------");
    println!("{}", second_answer);
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
        (Year::Year2022, Day::Day1) => print_answers(solver_2022_01::solve()),
        (Year::Year2022, Day::Day2) => print_answers(solver_2022_02::solve()),
        (Year::Year2022, Day::Day3) => print_answers(solver_2022_03::solve()),
        (Year::Year2022, Day::Day4) => print_answers(solver_2022_04::solve()),
        (Year::Year2022, Day::Day5) => print_answers(solver_2022_05::solve()),
        (Year::Year2022, Day::Day6) => print_answers(solver_2022_06::solve()),
        (Year::Year2022, Day::Day7) => print_answers(solver_2022_07::solve()),
        (Year::Year2022, Day::Day8) => print_answers(solver_2022_08::solve()),
        (Year::Year2022, Day::Day9) => print_answers(solver_2022_09::solve()),
        (Year::Year2022, Day::Day10) => print_answers(solver_2022_10::solve()),
        (Year::Year2022, Day::Day11) => print_answers(solver_2022_11::solve()),
        (Year::Year2022, Day::Day12) => print_answers(solver_2022_12::solve()),
        (Year::Year2022, Day::Day13) => print_answers(solver_2022_13::solve()),
        _ => panic!("Puzzle is not solved yet!")
    }
}
