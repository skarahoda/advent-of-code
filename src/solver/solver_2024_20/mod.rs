use super::Solver;
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
            cheat_threshold: 100,
        }
    }
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

    fn get_shortcuts(
        &self,
        cell: Coordinate,
        max_distance: usize,
        distances: &HashMap<Coordinate, usize>,
    ) -> usize {
        let signed_max_distance = max_distance as isize;

        let wrapper = || {
            let &current_distance = distances.get(&cell)?;
            Some(
                (-signed_max_distance..=signed_max_distance)
                    .map(|dx| {
                        let max_dy = signed_max_distance - dx.abs();
                        (-max_dy..=max_dy)
                            .flat_map(|dy| {
                                let new_cell = (
                                    cell.0.checked_add_signed(dx)?,
                                    cell.1.checked_add_signed(dy)?,
                                );
                                let new_distance = distances.get(&new_cell).cloned()?;
                                let distance_saved = new_distance.checked_sub(
                                    current_distance + dx.unsigned_abs() + dy.unsigned_abs(),
                                )?;
                                if distance_saved >= self.cheat_threshold {
                                    Some(())
                                } else {
                                    None
                                }
                            })
                            .count()
                    })
                    .sum::<usize>(),
            )
        };
        wrapper().unwrap_or(0)
    }
}

impl Solver<usize, usize> for Solver2024_20 {
    fn solve_first_part(&self) -> usize {
        let distances = self.get_distances();
        (0..self.map.len())
            .map(|y| {
                (0..self.map[0].len())
                    .map(|x| self.get_shortcuts((x, y), 2, &distances))
                    .sum::<usize>()
            })
            .sum()
    }

    fn solve_second_part(&self) -> usize {
        let distances = self.get_distances();
        (0..self.map.len())
            .map(|y| {
                (0..self.map[0].len())
                    .map(|x| self.get_shortcuts((x, y), 20, &distances))
                    .sum::<usize>()
            })
            .sum()
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
        let mut solver = Solver2024_20::from(EXAMPLE);
        solver.cheat_threshold = 50;
        assert_eq!(solver.solve_second_part(), 285);
    }
}
