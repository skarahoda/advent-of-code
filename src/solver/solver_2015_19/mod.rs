use super::Solver;
use std::collections::{HashMap, HashSet};

pub struct Solver2015_19<'a> {
    replacements: HashMap<&'a str, Vec<&'a str>>,
    input: &'a str,
}

impl Default for Solver2015_19<'_> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2015_19<'a> {
    fn from(input: &'a str) -> Self {
        let mut parts = input.split("\n\n");
        let mut replacements: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
        for line in parts.next().unwrap().lines() {
            let mut parts = line.split(" => ");
            replacements
                .entry(parts.next().unwrap())
                .or_default()
                .push(parts.next().unwrap());
        }
        Self {
            replacements,
            input: parts.next().unwrap(),
        }
    }
}

impl Solver<usize, usize> for Solver2015_19<'_> {
    fn solve_first_part(&self) -> usize {
        let mut outputs = HashSet::new();
        for (old_string, new_strings) in &self.replacements {
            for (i, _) in self.input.match_indices(old_string) {
                for new_string in new_strings {
                    let mut replaced = self.input.to_string();
                    replaced.replace_range(i..i + old_string.len(), new_string);
                    outputs.insert(replaced);
                }
            }
        }
        outputs.len()
    }

    fn solve_second_part(&self) -> usize {
        // Thanks to reddit user /u/askalski
        // https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4etju/
        let element_count = self
            .input
            .chars()
            .filter(|c| c.is_ascii_uppercase())
            .count();
        let rn_count = self.input.matches("Rn").count();
        let ar_count = self.input.matches("Ar").count();
        let y_count = self.input.matches("Y").count();
        element_count - rn_count - ar_count - (2 * y_count) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2015_19::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 4);
    }
}
