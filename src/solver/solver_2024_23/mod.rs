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

    fn get_one_more_bigger_parties(
        &'a self,
        parties: &HashSet<Vec<&'a str>>,
    ) -> HashSet<Vec<&'a str>> {
        let mut result = HashSet::new();
        for party in parties {
            for computer in self.computer_map.get(party.first().unwrap()).unwrap() {
                if party.contains(computer) {
                    continue;
                }
                if party.iter().all(|party_member| {
                    self.computer_map
                        .get(party_member)
                        .unwrap()
                        .contains(computer)
                }) {
                    let mut new_party = party.clone();
                    new_party.push(computer);
                    new_party.sort();
                    result.insert(new_party);
                }
            }
        }
        result
    }
}

impl Solver<usize, String> for Solver2024_23<'_> {
    fn solve_first_part(&self) -> usize {
        let mut mutated = self.clone();
        mutated.calculate_computer_map();
        let current_parties = mutated
            .computer_pairs
            .iter()
            .map(|&(a, b)| {
                let mut vec = vec![a, b];
                vec.sort();
                vec
            })
            .collect();
        let triplets = mutated.get_one_more_bigger_parties(&current_parties);

        triplets
            .iter()
            .filter(|triplet| triplet.iter().any(|computer| computer.starts_with("t")))
            .count()
    }

    fn solve_second_part(&self) -> String {
        let mut mutated = self.clone();
        mutated.calculate_computer_map();
        let mut current_parties = mutated
            .computer_pairs
            .iter()
            .map(|&(a, b)| {
                let mut vec = vec![a, b];
                vec.sort();
                vec
            })
            .collect();
        loop {
            let new_parties = mutated.get_one_more_bigger_parties(&current_parties);
            // println!("{:?}", new_parties);
            if new_parties.is_empty() {
                break;
            }
            current_parties = new_parties;
        }
        current_parties.iter().next().unwrap().join(",")
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

    #[test]
    fn test_solve_second_part() {
        let solver = Solver2024_23::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), "co,de,ka,ta");
    }
}
