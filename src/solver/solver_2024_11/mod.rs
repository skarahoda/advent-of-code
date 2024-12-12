use super::Solver;
use std::collections::HashMap;

mod input;
use input::INPUT;

#[derive(Clone)]
pub struct Solver2024_11 {
    rocks: HashMap<usize, usize>,
}

impl<'a> Default for Solver2024_11 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

impl<'a> From<&'a str> for Solver2024_11 {
    fn from(input: &'a str) -> Self {
        Self {
            rocks: input
                .split_whitespace()
                .map(|s| (s.parse().unwrap(), 1))
                .collect(),
        }
    }
}

impl Solver2024_11 {
    fn add(&mut self, rock: usize, count: usize) {
        let rock = self.rocks.entry(rock).or_insert(0);
        *rock += count;
    }

    fn blink(&self) -> Self {
        let mut result = Self {
            rocks: HashMap::new(),
        };
        for (&rock, &count) in self.rocks.iter() {
            let num_of_digits = if rock == 0 { 1 } else { rock.ilog10() + 1 };
            if rock == 0 {
                result.add(1, count);
            } else if num_of_digits % 2 == 0 {
                let divisor = 10_usize.pow(num_of_digits / 2);
                let left = rock / divisor;
                let right = rock % divisor;
                result.add(left, count);
                result.add(right, count);
            } else {
                result.add(rock * 2024, count);
            }
        }
        result
    }

    fn blink_n_times(&mut self, n: usize) {
        for _ in 0..n {
            *self = self.blink();
        }
    }
}

impl Solver<usize, usize> for Solver2024_11 {
    fn solve_first_part(&self) -> usize {
        let mut muted = self.clone();
        muted.blink_n_times(25);
        muted.rocks.values().sum()
    }

    fn solve_second_part(&self) -> usize {
        let mut muted = self.clone();
        muted.blink_n_times(75);
        muted.rocks.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "125 17";

    #[test]
    fn should_solve_first_part() {
        let solver = Solver2024_11::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 55312);
    }
}
