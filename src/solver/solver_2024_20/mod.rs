use crate::solver::Solver;
use std::collections::{HashMap, VecDeque};

type Coordinate = (usize, usize);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> Vec<Self> {
        vec![Self::Up, Self::Down, Self::Left, Self::Right]
    }
}

impl From<Direction> for (isize, isize) {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Clone)]
pub struct Solver2024_20 {
    map: Vec<Vec<bool>>,
    start: Coordinate,
    end: Coordinate,
    cheat_threshold: usize,
}

impl Default for Solver2024_20 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2024_20 {
    fn from(input: &str) -> Self {
        Self {
            map: input
                .lines()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect(),
            start: input
                .lines()
                .enumerate()
                .find_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .find_map(|(x, c)| (c == 'S').then(|| (x, y)))
                })
                .unwrap(),
            end: input
                .lines()
                .enumerate()
                .find_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .find_map(|(x, c)| (c == 'E').then(|| (x, y)))
                })
                .unwrap(),
            cheat_threshold: 100,
        }
    }
}

fn get_neighbor(
    location: Coordinate,
    direction: Direction,
    distances: &HashMap<Coordinate, usize>,
) -> Option<(Coordinate, usize)> {
    let (x, y) = location;
    let (dx, dy) = direction.into();
    let (x, y) = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);
    Some(((x, y), distances.get(&(x, y)).cloned()?))
}

impl Solver2024_20 {
    fn get_cell(&self, location: Coordinate, direction: Direction) -> Option<Coordinate> {
        let (x, y) = location;
        let (dx, dy) = direction.into();
        let (x, y) = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);
        if *self.map.get(y)?.get(x)? == false {
            Some((x, y))
        } else {
            None
        }
    }
    fn get_distances(&self) -> HashMap<Coordinate, usize> {
        let mut queue: VecDeque<Coordinate> = vec![self.start].into();
        let mut distances: HashMap<Coordinate, usize> = HashMap::new();
        distances.insert(self.start, 0);
        while let Some(current) = queue.pop_front() {
            let current_distance = distances.get(&current).cloned().unwrap_or(0);
            for direction in Direction::all() {
                if let Some(next) = self.get_cell(current, direction) {
                    if distances.contains_key(&next) {
                        continue;
                    }
                    queue.push_back(next);
                    distances.insert(next, current_distance + 1);
                }
            }
        }
        distances
    }

    fn is_cell_above_cheat_threshold(
        &self,
        (x, y): Coordinate,
        distances: &HashMap<Coordinate, usize>,
    ) -> bool {
        if self.map[y][x] == false {
            return false;
        }
        let neighbors: Vec<(Coordinate, usize)> = Direction::all()
            .into_iter()
            .filter_map(|direction| get_neighbor((x, y), direction, &distances))
            .collect();
        if neighbors.is_empty() {
            return false;
        }
        let max_neighbor = neighbors.iter().max_by_key(|(_, d)| *d).unwrap();
        let min_neighbor = neighbors.iter().min_by_key(|(_, d)| *d).unwrap();
        if let Some(diff) = max_neighbor.1.checked_sub(min_neighbor.1 + 2) {
            diff >= self.cheat_threshold
        } else {
            false
        }
    }
}

impl Solver<usize, usize> for Solver2024_20 {
    fn solve_first_part(&self) -> usize {
        let mut result = 0;
        let distances = self.get_distances();
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if self.is_cell_above_cheat_threshold((x, y), &distances) {
                    result += 1;
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
        let mut solver = Solver2024_20::from(EXAMPLE);
        solver.cheat_threshold = 1;
        assert_eq!(solver.solve_first_part(), 44);
    }

    #[test]
    fn test_solve_second_part() {
        let solver = Solver2024_20::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 1);
    }
}
