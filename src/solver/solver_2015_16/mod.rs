use super::Solver;
use std::collections::HashMap;

pub struct Solver2015_16<'a> {
    gifts: HashMap<usize, HashMap<&'a str, usize>>,
    goal: HashMap<&'a str, usize>,
}

impl Default for Solver2015_16<'_> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2015_16<'a> {
    fn from(input: &'a str) -> Self {
        // Sue 1: goldfish: 6, trees: 9, akitas: 0

        let re = regex::Regex::new(r"Sue (?P<id>\d+):(?P<rest>.*)").unwrap();
        Self {
            gifts: re
                .captures_iter(input)
                .map(|captures: regex::Captures| {
                    (
                        captures.name("id").unwrap().as_str().parse().unwrap(),
                        captures
                            .name("rest")
                            .unwrap()
                            .as_str()
                            .split(", ")
                            .map(|s| {
                                let re =
                                    regex::Regex::new(r"(?P<name>\w+): (?P<count>\d+)").unwrap();
                                (
                                    re.captures(s).unwrap().name("name").unwrap().as_str(),
                                    re.captures(s)
                                        .unwrap()
                                        .name("count")
                                        .unwrap()
                                        .as_str()
                                        .parse()
                                        .unwrap(),
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),
            goal: HashMap::from([
                ("children", 3),
                ("cats", 7),
                ("samoyeds", 2),
                ("pomeranians", 3),
                ("akitas", 0),
                ("vizslas", 0),
                ("goldfish", 5),
                ("trees", 3),
                ("cars", 2),
                ("perfumes", 1),
            ]),
        }
    }
}

impl Solver<usize, usize> for Solver2015_16<'_> {
    fn solve_first_part(&self) -> usize {
        let sue = self.gifts.iter().find(|(_, gifts)| {
            gifts
                .iter()
                .all(|(name, count)| self.goal.get(name).unwrap() == count)
        });
        sue.unwrap().0.clone()
    }

    fn solve_second_part(&self) -> usize {
        let sue = self.gifts.iter().find(|(_, gifts)| {
            gifts.iter().all(|(name, count)| {
                let goal = self.goal.get(name).unwrap();
                match *name {
                    "cats" | "trees" => count > goal,
                    "pomeranians" | "goldfish" => count < goal,
                    _ => count == goal,
                }
            })
        });
        sue.unwrap().0.clone()
    }
}
