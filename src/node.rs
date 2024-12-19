use super::solver::{
    solver_2015_01, solver_2015_02, solver_2015_03, solver_2015_04, solver_2015_05, solver_2015_06,
    solver_2015_07, solver_2022_01, solver_2022_02, solver_2022_03, solver_2022_04, solver_2022_05,
    solver_2022_06, Solver, Solver2015_08, Solver2022_07, Solver2022_08, Solver2022_09,
    Solver2022_10, Solver2022_11, Solver2022_12, Solver2022_13, Solver2022_14, Solver2022_15,
    Solver2022_16, Solver2022_17, Solver2022_18, Solver2022_20, Solver2022_21, Solver2022_22,
    Solver2024_01, Solver2024_02, Solver2024_03, Solver2024_04, Solver2024_05, Solver2024_06,
    Solver2024_07, Solver2024_08, Solver2024_09, Solver2024_10, Solver2024_11, Solver2024_12,
    Solver2024_13, Solver2024_14, Solver2024_15, Solver2024_16, Solver2024_17,
};
use napi_derive::napi;
use std::fmt::Display;

#[napi(object)]
pub struct Answers {
    pub first: String,
    pub second: String,
}

impl<T1: Display, T2: Display> From<(T1, T2)> for Answers {
    fn from((first, second): (T1, T2)) -> Self {
        Self {
            first: format!("{first}"),
            second: format!("{second}"),
        }
    }
}

impl<T1: Display, T2: Display> From<Box<dyn Solver<T1, T2>>> for Answers {
    fn from(solver: Box<dyn Solver<T1, T2>>) -> Self {
        Self {
            first: format!("{}", solver.solve_first_part()),
            second: format!("{}", solver.solve_second_part()),
        }
    }
}

#[napi]
pub fn solve(year: u32, day: u32) -> Answers {
    match (year, day) {
        (2015, 1) => Answers::from(solver_2015_01::solve()),
        (2015, 2) => Answers::from(solver_2015_02::solve()),
        (2015, 3) => Answers::from(solver_2015_03::solve()),
        (2015, 4) => Answers::from(solver_2015_04::solve()),
        (2015, 5) => Answers::from(solver_2015_05::solve()),
        (2015, 6) => Answers::from(solver_2015_06::solve()),
        (2015, 7) => Answers::from(solver_2015_07::solve()),
        (2015, 8) => Answers::from(Box::new(Solver2015_08::default()) as Box<dyn Solver<_, _>>),
        (2022, 1) => Answers::from(solver_2022_01::solve()),
        (2022, 2) => Answers::from(solver_2022_02::solve()),
        (2022, 3) => Answers::from(solver_2022_03::solve()),
        (2022, 4) => Answers::from(solver_2022_04::solve()),
        (2022, 5) => Answers::from(solver_2022_05::solve()),
        (2022, 6) => Answers::from(solver_2022_06::solve()),
        (2022, 7) => Answers::from(Box::new(Solver2022_07::default()) as Box<dyn Solver<_, _>>),
        (2022, 8) => Answers::from(Box::new(Solver2022_08::default()) as Box<dyn Solver<_, _>>),
        (2022, 9) => Answers::from(Box::new(Solver2022_09::default()) as Box<dyn Solver<_, _>>),
        (2022, 10) => Answers::from(Box::new(Solver2022_10::default()) as Box<dyn Solver<_, _>>),
        (2022, 11) => Answers::from(Box::new(Solver2022_11::default()) as Box<dyn Solver<_, _>>),
        (2022, 12) => Answers::from(Box::new(Solver2022_12::default()) as Box<dyn Solver<_, _>>),
        (2022, 13) => Answers::from(Box::new(Solver2022_13::default()) as Box<dyn Solver<_, _>>),
        (2022, 14) => Answers::from(Box::new(Solver2022_14::default()) as Box<dyn Solver<_, _>>),
        (2022, 15) => Answers::from(Box::new(Solver2022_15::default()) as Box<dyn Solver<_, _>>),
        (2022, 16) => Answers::from(Box::new(Solver2022_16::default()) as Box<dyn Solver<_, _>>),
        (2022, 17) => Answers::from(Box::new(Solver2022_17::default()) as Box<dyn Solver<_, _>>),
        (2022, 18) => Answers::from(Box::new(Solver2022_18::default()) as Box<dyn Solver<_, _>>),
        (2022, 20) => Answers::from(Box::new(Solver2022_20::default()) as Box<dyn Solver<_, _>>),
        (2022, 21) => Answers::from(Box::new(Solver2022_21::default()) as Box<dyn Solver<_, _>>),
        (2022, 22) => Answers::from(Box::new(Solver2022_22::default()) as Box<dyn Solver<_, _>>),
        (2024, 1) => Answers::from(Box::new(Solver2024_01::default()) as Box<dyn Solver<_, _>>),
        (2024, 2) => Answers::from(Box::new(Solver2024_02::default()) as Box<dyn Solver<_, _>>),
        (2024, 3) => Answers::from(Box::new(Solver2024_03::default()) as Box<dyn Solver<_, _>>),
        (2024, 4) => Answers::from(Box::new(Solver2024_04::default()) as Box<dyn Solver<_, _>>),
        (2024, 5) => Answers::from(Box::new(Solver2024_05::default()) as Box<dyn Solver<_, _>>),
        (2024, 6) => Answers::from(Box::new(Solver2024_06::default()) as Box<dyn Solver<_, _>>),
        (2024, 7) => Answers::from(Box::new(Solver2024_07::default()) as Box<dyn Solver<_, _>>),
        (2024, 8) => Answers::from(Box::new(Solver2024_08::default()) as Box<dyn Solver<_, _>>),
        (2024, 9) => Answers::from(Box::new(Solver2024_09::default()) as Box<dyn Solver<_, _>>),
        (2024, 10) => Answers::from(Box::new(Solver2024_10::default()) as Box<dyn Solver<_, _>>),
        (2024, 11) => Answers::from(Box::new(Solver2024_11::default()) as Box<dyn Solver<_, _>>),
        (2024, 12) => Answers::from(Box::new(Solver2024_12::default()) as Box<dyn Solver<_, _>>),
        (2024, 13) => Answers::from(Box::new(Solver2024_13::default()) as Box<dyn Solver<_, _>>),
        (2024, 14) => Answers::from(Box::new(Solver2024_14::default()) as Box<dyn Solver<_, _>>),
        (2024, 15) => Answers::from(Box::new(Solver2024_15::default()) as Box<dyn Solver<_, _>>),
        (2024, 16) => Answers::from(Box::new(Solver2024_16::default()) as Box<dyn Solver<_, _>>),
        (2024, 17) => Answers::from(Box::new(Solver2024_17::default()) as Box<dyn Solver<_, _>>),
        _ => Answers {
            first: "hello".to_string(),
            second: "bar".to_string(),
        },
    }
}
