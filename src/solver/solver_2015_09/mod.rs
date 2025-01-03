use super::Solver;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Solver2015_09<'a> {
    distances: HashMap<(&'a str, &'a str), usize>,
    neighbors: HashMap<&'a str, HashSet<&'a str>>,
    all_cities: HashSet<&'a str>,
}

impl Default for Solver2015_09<'_> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2015_09<'a> {
    fn from(input: &'a str) -> Self {
        let re = Regex::new(r"(?P<city1>\w+) to (?P<city2>\w+) = (?P<distance>\d+)").unwrap();
        let distances: HashMap<(&str, &str), usize> = input
            .lines()
            .map(|s| {
                let captures = re.captures(s).unwrap();

                let city1 = captures.name("city1").unwrap().as_str();
                let city2 = captures.name("city2").unwrap().as_str();
                let distance = captures.name("distance").unwrap().as_str().parse().unwrap();
                ((city1, city2), distance)
            })
            .collect();
        let mut neighbors: HashMap<&str, HashSet<&str>> = HashMap::new();
        let mut all_cities: HashSet<&str> = HashSet::new();
        for (&(a, b), _) in distances.iter() {
            neighbors.entry(a).or_default().insert(b);
            neighbors.entry(b).or_default().insert(a);
            all_cities.insert(a);
            all_cities.insert(b);
        }
        Self {
            distances,
            neighbors,
            all_cities,
        }
    }
}

impl<'a> Solver2015_09<'a> {
    fn get_distance(&self, city1: &str, city2: &str) -> Option<usize> {
        self.distances
            .get(&(city1, city2))
            .or_else(|| self.distances.get(&(city2, city1)))
            .cloned()
    }
    fn get_shortest_distance<'b>(
        &self,
        current: &str,
        visited: &'b mut HashSet<&'a str>,
    ) -> Option<usize> {
        if visited.len() == self.all_cities.len() {
            return Some(0);
        }
        let mut result = None;
        for next in self.neighbors.get(current)? {
            if visited.contains(next) {
                continue;
            }
            visited.insert(next);
            let distance = self
                .get_shortest_distance(next, visited)
                .and_then(|distance| self.get_distance(current, next).map(|d| distance + d));
            if let Some(distance) = distance {
                if result.is_none_or(|result| distance < result) {
                    result = Some(distance);
                }
            }
            visited.remove(next);
        }
        result
    }

    fn get_longest_distance<'b>(
        &self,
        current: &str,
        visited: &'b mut HashSet<&'a str>,
    ) -> Option<usize> {
        if visited.len() == self.all_cities.len() {
            return Some(0);
        }
        let mut result = None;
        for next in self.neighbors.get(current)? {
            if visited.contains(next) {
                continue;
            }
            visited.insert(next);
            let distance = self
                .get_longest_distance(next, visited)
                .and_then(|distance| self.get_distance(current, next).map(|d| distance + d));
            if let Some(distance) = distance {
                if result.is_none_or(|result| distance > result) {
                    result = Some(distance);
                }
            }
            visited.remove(next);
        }
        result
    }
}

impl Solver<usize, usize> for Solver2015_09<'_> {
    fn solve_first_part(&self) -> usize {
        self.all_cities
            .iter()
            .filter_map(|&city| {
                let mut visited = HashSet::new();
                visited.insert(city);
                self.get_shortest_distance(city, &mut visited)
            })
            .min()
            .unwrap()
    }

    fn solve_second_part(&self) -> usize {
        self.all_cities
            .iter()
            .filter_map(|&city| {
                let mut visited = HashSet::new();
                visited.insert(city);
                self.get_longest_distance(city, &mut visited)
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2015_09::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 605);
    }

    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2015_09::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 982);
    }
}
