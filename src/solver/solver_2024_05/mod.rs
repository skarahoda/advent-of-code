use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
mod input;
use super::Solver;
use input::INPUT;

pub struct Solver202405 {
    precedence_map: HashMap<i32, HashSet<i32>>,
    prints: Vec<Vec<i32>>,
}

fn get_middle<T>(array: &Vec<T>) -> Option<&T> {
    array.get((array.len() - 1) / 2)
}
impl Solver202405 {
    fn is_valid_print(&self, print: &Vec<i32>) -> bool {
        let mut visited = HashSet::new();
        for &x in print {
            let precedences = self.precedence_map.get(&x);
            if precedences
                .unwrap_or(&HashSet::new())
                .iter()
                .any(|y| visited.contains(y))
            {
                return false;
            }
            visited.insert(x);
        }
        true
    }

    fn sort_print(&self, print: &mut Vec<i32>) {
        print.sort_by(|a, b| {
            let a_precedences = self.precedence_map.get(a);
            let b_precedences = self.precedence_map.get(b);
            if a_precedences.is_some() && a_precedences.unwrap().contains(b) {
                Ordering::Less
            } else if b_precedences.is_some() && b_precedences.unwrap().contains(a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
    }
}

impl From<&str> for Solver202405 {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split("\n\n").collect();
        let precedence_map = parts[0].lines().fold(HashMap::new(), |mut map, line| {
            let nums: Vec<&str> = line.split('|').collect();
            let x = nums[0].parse().unwrap();
            let y = nums[1].parse().unwrap();
            map.entry(x).or_insert_with(HashSet::new).insert(y);
            map
        });
        let prints = parts[1]
            .lines()
            .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
            .collect();
        Self {
            precedence_map,
            prints,
        }
    }
}

impl Default for Solver202405 {
    fn default() -> Self {
        INPUT.into()
    }
}

impl Solver<i32, i32> for Solver202405 {
    fn solve_first_part(&self) -> i32 {
        self.prints
            .iter()
            .filter(|print| self.is_valid_print(print))
            .map(|print| *get_middle(&print).unwrap())
            .sum()
    }

    fn solve_second_part(&self) -> i32 {
        self.prints
            .iter()
            .filter(|print| !self.is_valid_print(print))
            .map(|print| {
                let mut print = print.clone();
                self.sort_print(&mut print);
                *get_middle(&print).unwrap()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn solve_first_part() {
        assert_eq!(Solver202405::from(EXAMPLE).solve_first_part(), 143);
    }

    #[test]
    fn solve_second_part() {
        assert_eq!(Solver202405::from(EXAMPLE).solve_second_part(), 123);
    }
}
