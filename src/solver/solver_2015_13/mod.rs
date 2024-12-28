use super::Solver;
use std::collections::{HashMap, HashSet};

pub struct Solver2015_13<'a> {
    happiness_scores: HashMap<(&'a str, &'a str), isize>,
    people: HashSet<&'a str>,
}

impl Default for Solver2015_13<'_> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2015_13<'a> {
    fn from(input: &'a str) -> Self {
        let re = regex::Regex::new(r"(?P<first>\w+) would (?P<action>gain|lose) (?P<value>\d+) happiness units by sitting next to (?P<second>\w+).").unwrap();
        let mut people: HashSet<&str> = HashSet::new();
        let happiness_scores: HashMap<(&'a str, &'a str), isize> = input
            .lines()
            .map(|line| {
                let captures = re.captures(line).unwrap();
                let score: isize = captures.name("value").unwrap().as_str().parse().unwrap();
                let first = captures.name("first").unwrap().as_str();
                let second = captures.name("second").unwrap().as_str();
                people.insert(first);
                people.insert(second);
                (
                    (first, second),
                    if captures.name("action").unwrap().as_str() == "gain" {
                        score
                    } else {
                        -score
                    },
                )
            })
            .collect();

        Self {
            happiness_scores,
            people,
        }
    }
}

impl<'a> Solver2015_13<'a> {
    fn get_happiness_score(&self, first: &'a str, second: &'a str) -> isize {
        let pair = (first, second);
        let first_score = self.happiness_scores.get(&pair).unwrap_or(&0);
        let pair = (second, first);
        let second_score = self.happiness_scores.get(&pair).unwrap_or(&0);
        *first_score + *second_score
    }

    fn get_highest_happiness_score_of_table(
        &self,
        first: &'a str,
        current: &'a str,
        placed: &mut HashSet<&'a str>,
    ) -> isize {
        if placed.len() == self.people.len() {
            return self.get_happiness_score(first, current);
        }
        let mut result = isize::MIN;
        for &next in &self.people {
            if placed.contains(next) {
                continue;
            }
            placed.insert(next);
            let score = self.get_highest_happiness_score_of_table(first, next, placed)
                + self.get_happiness_score(current, next);
            if score > result {
                result = score;
            }
            placed.remove(next);
        }
        result
    }
}

impl Solver<isize, isize> for Solver2015_13<'_> {
    fn solve_first_part(&self) -> isize {
        let &first = self.people.iter().next().unwrap();
        let mut placed = HashSet::new();
        placed.insert(first);
        let score = self.get_highest_happiness_score_of_table(first, first, &mut placed);
        score
    }

    fn solve_second_part(&self) -> isize {
        let santa = "";
        let score = self.get_highest_happiness_score_of_table(santa, santa, &mut HashSet::new());
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2015_13::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 330);
    }
}
