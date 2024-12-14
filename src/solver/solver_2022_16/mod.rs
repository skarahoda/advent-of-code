use super::Solver;
mod input;
use input::INPUT;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq, Debug)]
struct Valve {
    flow: i32,
    tunnels: HashSet<String>,
}

impl Valve {
    fn new(flow: i32, tunnels: HashSet<String>) -> Self {
        Self { flow, tunnels }
    }
}

type Valves = HashMap<String, Valve>;

fn get_max_possible_remaining_score(
    remaining_times: (i32, i32),
    mut remaining_flows: Vec<i32>,
) -> i32 {
    let max_time = max(remaining_times.0, remaining_times.1);
    let min_time = min(remaining_times.0, remaining_times.1);
    if max_time < 2 || remaining_flows.len() == 0 {
        0
    } else {
        let reduced_remaining_time = max_time - 2;
        let next_flow = remaining_flows.pop().unwrap();
        (reduced_remaining_time * next_flow)
            + get_max_possible_remaining_score((reduced_remaining_time, min_time), remaining_flows)
    }
}

pub struct Solver2022_16 {
    valves: Valves,
}

impl Default for Solver2022_16 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

impl<'a> From<&'a str> for Solver2022_16 {
    fn from(input: &'a str) -> Self {
        let re = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();

        Self {
            valves: input
                .split("\n")
                .map(|row| {
                    let captures = re.captures(row).unwrap();
                    (
                        captures.get(1).unwrap().as_str().to_string(),
                        Valve::new(
                            captures.get(2).unwrap().as_str().parse().unwrap(),
                            captures
                                .get(3)
                                .unwrap()
                                .as_str()
                                .split(", ")
                                .map(|s| String::from(s))
                                .collect(),
                        ),
                    )
                })
                .collect(),
        }
    }
}

impl Solver2022_16 {
    fn get_shortest_paths(&self, root: &str) -> HashMap<String, i32> {
        let mut result: HashMap<String, i32> = HashMap::new();
        let mut distances: HashMap<String, i32> = HashMap::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        distances.insert(root.to_string(), 0);
        queue.push_back(root.to_string());

        while let Some(current_point) = queue.pop_front() {
            let neighbours = &self.valves.get(&current_point).unwrap().tunnels;
            let current_distance = *distances.get(&current_point).unwrap();
            for neighbour in neighbours {
                if !distances.contains_key(neighbour) {
                    distances.insert(neighbour.to_string(), current_distance + 1);
                    queue.push_back(neighbour.to_string());
                    if self.valves.get(neighbour).unwrap().flow > 0 {
                        result.insert(neighbour.to_string(), current_distance + 1);
                    }
                }
            }
        }
        result
    }

    fn get_shortest_paths_map(&self) -> HashMap<&str, HashMap<String, i32>> {
        self.valves
            .keys()
            .map(|point| (point.as_str(), self.get_shortest_paths(point)))
            .collect()
    }
}

impl Solver<i32, i32> for Solver2022_16 {
    fn solve_first_part(&self) -> i32 {
        let mut candidates: Vec<(i32, i32, &str, HashSet<String>)> = Vec::new();
        let shortest_path_map = self.get_shortest_paths_map();
        candidates.push((0, 30, "AA", HashSet::from(["AA".to_string()])));
        let mut winner = 0;

        while let Some((score, remaining_minutes, current_point, visited_points)) = candidates.pop()
        {
            let shortest_paths = shortest_path_map.get(current_point).unwrap();
            for (next_point, distance) in shortest_paths {
                let remaining_minutes_after_open = remaining_minutes - distance - 1;
                if !visited_points.contains(next_point) && remaining_minutes_after_open > 0 {
                    let score_for_valve =
                        remaining_minutes_after_open * self.valves.get(next_point).unwrap().flow;
                    let mut new_visited_points = visited_points.clone();
                    new_visited_points.insert(next_point.to_string());
                    let next_score = score + score_for_valve;
                    candidates.push((
                        score + score_for_valve,
                        remaining_minutes_after_open,
                        next_point,
                        new_visited_points,
                    ));
                    if next_score > winner {
                        winner = next_score;
                    }
                }
            }
        }
        winner
    }

    fn solve_second_part(&self) -> i32 {
        let mut candidates: Vec<(i32, (i32, &str), (i32, &str), HashSet<String>)> = Vec::new();
        let shortest_path_map = self.get_shortest_paths_map();
        let mut sorted_flows: Vec<(&str, i32)> = self
            .valves
            .iter()
            .filter(|&(_, valve)| valve.flow > 0)
            .map(|(name, valve)| (name.as_str(), valve.flow))
            .collect();
        sorted_flows.sort();
        candidates.push((0, (26, "AA"), (26, "AA"), HashSet::from(["AA".to_string()])));
        let mut winner = 0;

        while let Some((score, player_state, elephant_state, visited_points)) = candidates.pop() {
            for (remaining_minutes, current_point, other_state) in [
                (player_state.0, player_state.1, elephant_state),
                (elephant_state.0, elephant_state.1, player_state),
            ] {
                let shortest_paths = shortest_path_map.get(current_point).unwrap();
                for (next_point, distance) in shortest_paths {
                    let remaining_minutes_after_open = remaining_minutes - distance - 1;
                    let score_for_valve =
                        remaining_minutes_after_open * self.valves.get(next_point).unwrap().flow;
                    let next_score = score + score_for_valve;
                    let remaining_flows: Vec<i32> = sorted_flows
                        .iter()
                        .filter(|(valve_name, _)| {
                            valve_name != next_point && !visited_points.contains(*valve_name)
                        })
                        .map(|&(_, flow)| flow)
                        .collect();
                    let max_possible_score = next_score
                        + get_max_possible_remaining_score(
                            (remaining_minutes_after_open, other_state.0),
                            remaining_flows,
                        );
                    if max_possible_score > winner
                        && !visited_points.contains(next_point)
                        && remaining_minutes_after_open > 0
                    {
                        let mut new_visited_points = visited_points.clone();
                        new_visited_points.insert(next_point.to_string());
                        candidates.push((
                            next_score,
                            (remaining_minutes_after_open, next_point),
                            other_state,
                            new_visited_points,
                        ));
                        if next_score > winner {
                            winner = next_score;
                        }
                    }
                }
            }
        }
        winner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
        Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
        Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
        Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
        Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
        Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
        Valve HH has flow rate=22; tunnel leads to valve GG\n\
        Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
        Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn should_solve_first_part() {
        let solver = Solver2022_16::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 1651);
    }

    #[test]
    fn should_solve_second_part() {
        let solver = Solver2022_16::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 1707);
    }
}
