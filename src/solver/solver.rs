use std::fmt::Display;

pub trait Solver<T1: Display, T2: Display> {
    fn solve_first_part(&self) -> T1;
    fn solve_second_part(&self) -> T2;
}
