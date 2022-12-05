extern crate core;

use clap::{Parser, ValueEnum};
mod solver;
use solver::{
    solver_2015_01,
    solver_2022_01,
    solver_2022_02,
    solver_2022_03,
    solver_2022_04,
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
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_enum)]
    year: Year,

    #[arg(value_enum)]
    day: Day,

    #[arg(short, long)]
    second_part: bool,
}

fn main() {
    let args = Cli::parse();
    match (args.year, args.day, args.second_part) {
        (Year::Year2015, Day::Day1, false) => println!("{}", solver_2015_01::solve_first_part()),
        (Year::Year2015, Day::Day1, true) => println!("{}", solver_2015_01::solve_second_part()),
        (Year::Year2022, Day::Day1, false) => println!("{}", solver_2022_01::solve_first_part()),
        (Year::Year2022, Day::Day1, true) => println!("{}", solver_2022_01::solve_second_part()),
        (Year::Year2022, Day::Day2, false) => println!("{}", solver_2022_02::solve_first_part()),
        (Year::Year2022, Day::Day2, true) => println!("{}", solver_2022_02::solve_second_part()),
        (Year::Year2022, Day::Day3, false) => println!("{}", solver_2022_03::solve_first_part()),
        (Year::Year2022, Day::Day3, true) => println!("{}", solver_2022_03::solve_second_part()),
        (Year::Year2022, Day::Day4, false) => println!("{}", solver_2022_04::solve_first_part()),
        (Year::Year2022, Day::Day4, true) => println!("{}", solver_2022_04::solve_second_part()),
        _ => panic!("Puzzle is not solved yet!")
    }
}
