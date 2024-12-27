use super::Solver;
use md5;

pub struct Solver2015_04<'a> {
    input: &'a str,
}

impl Default for Solver2015_04<'_> {
    fn default() -> Self {
        Self::from("bgvyzdsv")
    }
}

impl<'a> From<&'a str> for Solver2015_04<'a> {
    fn from(input: &'a str) -> Self {
        Self { input }
    }
}

impl Solver<u32, u32> for Solver2015_04<'_> {
    fn solve_first_part(&self) -> u32 {
        let mut number: u32 = 0;
        loop {
            let hash = md5::compute(format!("{}{}", self.input, number));
            if format!("{:x}", hash).starts_with("00000") {
                return number;
            }
            number += 1;
        }
    }

    fn solve_second_part(&self) -> u32 {
        let mut number: u32 = 0;
        loop {
            let hash = md5::compute(format!("{}{}", self.input, number));
            if format!("{:x}", hash).starts_with("000000") {
                return number;
            }
            number += 1;
        }
    }
}

#[cfg(test)]
mod first_part {
    use super::*;
    #[test]
    fn solve_first_example() {
        let solver = Solver2015_04::from("abcdef");
        assert_eq!(solver.solve_first_part(), 609043);
    }
    #[test]
    fn solve_second_example() {
        let solver = Solver2015_04::from("pqrstuv");
        assert_eq!(solver.solve_first_part(), 1048970);
    }
}
