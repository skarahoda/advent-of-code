use super::Solver;
use std::collections::{HashMap, HashSet};
#[derive(Clone)]
pub struct Solver2024_23<'a> {
    computer_pairs: Vec<(&'a str, &'a str)>,
    computer_map: HashMap<&'a str, HashSet<&'a str>>,
}

impl Default for Solver2024_23<'_> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2024_23<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            computer_pairs: input
                .lines()
                .map(|line| {
                    let mut computers = line.split("-");
                    (computers.next().unwrap(), computers.next().unwrap())
                })
                .collect(),
            computer_map: HashMap::new(),
        }
    }
}

impl<'a> Solver2024_23<'a> {
    fn calculate_computer_map(&mut self) {
        for (computer1, computer2) in &self.computer_pairs {
            self.computer_map
                .entry(computer1)
                .or_insert_with(HashSet::new)
                .insert(computer2);
            self.computer_map
                .entry(computer2)
                .or_insert_with(HashSet::new)
                .insert(computer1);
        }
    }

    fn get_triplets(&self) -> HashSet<(&str, &str, &str)> {
        let mut result = HashSet::new();
        for (&first_computer, neighbors) in self.computer_map.iter() {
            for &second_computer in neighbors {
                for &third_computer in self.computer_map.get(second_computer).unwrap() {
                    if neighbors.contains(third_computer) {
                        let mut vec = vec![first_computer, second_computer, third_computer];
                        vec.sort();
                        result.insert((vec[0], vec[1], vec[2]));
                    }
                }
            }
        }
        result
    }
}

impl Solver<usize, usize> for Solver2024_23<'_> {
    fn solve_first_part(&self) -> usize {
        let mut mutated = self.clone();
        mutated.calculate_computer_map();
        mutated
            .get_triplets()
            .iter()
            .filter(|(first, second, third)| {
                first.starts_with("t") || second.starts_with("t") || third.starts_with("t")
            })
            .count()
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
        let solver = Solver2024_23::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 7);
    }
}
