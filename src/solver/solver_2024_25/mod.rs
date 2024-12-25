use super::Solver;
use regex::Regex;
use std::collections::HashMap;

pub struct Solver2024_25 {
    keys: HashMap<[usize; 5], usize>,
    locks: HashMap<[usize; 5], usize>,
}

impl Default for Solver2024_25 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2024_25 {
    fn from(input: &str) -> Self {
        let mut keys = HashMap::new();
        let mut locks = HashMap::new();
        for key_or_lock in input.split("\n\n") {
            let is_lock = key_or_lock.starts_with("#");
            let mut shape = [0; 5];
            let re_first_and_last_line = Regex::new(r"(\n.*$|^.*\n)").unwrap();
            let stripped = re_first_and_last_line.replace_all(key_or_lock, "");
            for (i, c) in stripped.chars().enumerate() {
                if c == '#' {
                    shape[i % 6] += 1;
                }
            }
            if is_lock {
                *locks.entry(shape).or_default() += 1;
            } else {
                *keys.entry(shape).or_default() += 1;
            }
        }
        Self { keys, locks }
    }
}

impl Solver<usize, usize> for Solver2024_25 {
    fn solve_first_part(&self) -> usize {
        let mut result = 0;
        for (key, key_count) in &self.keys {
            for (lock, lock_count) in &self.locks {
                if (0..5).all(|i| key[i] + lock[i] <= 5) {
                    result += key_count * lock_count;
                }
            }
        }
        result
    }

    fn solve_second_part(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn test_solve_first_part() {
        let solver = Solver2024_25::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 3);
    }

    #[test]
    fn test_solve_second_part() {
        let solver = Solver2024_25::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 0);
    }
}
