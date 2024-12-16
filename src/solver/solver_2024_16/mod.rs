use super::Solver;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

enum Cell {
    Empty,
    Wall,
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn rotate_counter_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}
#[derive(PartialEq, Clone, Debug, Hash, Eq)]
struct Coordinate(usize, usize);

impl Coordinate {
    fn next(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self(self.0, self.1 - 1),
            Direction::Down => Self(self.0, self.1 + 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Right => Self(self.0 + 1, self.1),
        }
    }
}

pub struct Solver2024_16 {
    maze: Vec<Vec<Cell>>,
    start: Coordinate,
    end: Coordinate,
}

impl Default for Solver2024_16 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2024_16 {
    fn from(input: &str) -> Self {
        Self {
            maze: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' | 'S' | 'E' => Cell::Empty,
                            '#' => Cell::Wall,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
            start: input
                .lines()
                .enumerate()
                .find_map(|(y, line)| {
                    line.chars().enumerate().find_map(|(x, c)| match c {
                        'S' => Some(Coordinate(x, y)),
                        _ => None,
                    })
                })
                .unwrap(),
            end: input
                .lines()
                .enumerate()
                .find_map(|(y, line)| {
                    line.chars().enumerate().find_map(|(x, c)| match c {
                        'E' => Some(Coordinate(x, y)),
                        _ => None,
                    })
                })
                .unwrap(),
        }
    }
}

fn get_min_of_coordinate(
    coordinate: &Coordinate,
    map: &HashMap<(Coordinate, Direction), usize>,
) -> Option<usize> {
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    .filter_map(|direction| map.get(&(coordinate.clone(), direction.clone())))
    .min()
    .cloned()
}

impl Solver2024_16 {
    fn is_valid(&self, coordinate: &Coordinate) -> bool {
        let wrapper = || -> Option<()> {
            let cell = self.maze.get(coordinate.1)?.get(coordinate.0)?;
            match cell {
                Cell::Empty => Some(()),
                Cell::Wall => None,
            }
        };
        wrapper().is_some()
    }

    fn find_shortest_path(&self) -> Option<usize> {
        let mut processed_cells: HashSet<(Coordinate, Direction)> = HashSet::new();
        let mut map: HashMap<(Coordinate, Direction), usize> = HashMap::new();

        map.insert((self.start.clone(), Direction::Up), 1000);
        map.insert((self.start.clone(), Direction::Down), 1000);
        map.insert((self.start.clone(), Direction::Left), 2000);
        map.insert((self.start.clone(), Direction::Right), 0);

        while processed_cells.len() < map.len() {
            let ((min_cell, direction), score) = map
                .iter()
                .filter(|(key, _)| !processed_cells.contains(key))
                .min_by_key(|(_, value)| *value)
                .unwrap();
            let min_cell = min_cell.clone();
            let direction = direction.clone();
            let score = score.clone();

            processed_cells.insert((min_cell.clone(), direction.clone()));

            let next_cell = &min_cell.next(&direction);
            if self.is_valid(next_cell) {
                let next_score = score + 1;
                map.entry((next_cell.clone(), direction.clone()))
                    .and_modify(|value| {
                        *value = next_score.min(*value);
                    })
                    .or_insert(next_score);
                let next_score = next_score + 1000;
                map.entry((next_cell.clone(), direction.rotate_clockwise()))
                    .and_modify(|value| {
                        *value = next_score.min(*value);
                    })
                    .or_insert(next_score);
                map.entry((next_cell.clone(), direction.rotate_counter_clockwise()))
                    .and_modify(|value| {
                        *value = next_score.min(*value);
                    })
                    .or_insert(next_score);
            }
        }
        get_min_of_coordinate(&self.end, &map)
    }
}

impl Solver<usize, usize> for Solver2024_16 {
    fn solve_first_part(&self) -> usize {
        self.find_shortest_path().unwrap()
    }

    fn solve_second_part(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############\
";

    #[test]
    fn should_solve_first_part() {
        let solver = Solver2024_16::from(EXAMPLE);
        assert_eq!(solver.find_shortest_path().unwrap(), 7036);
    }

    #[test]
    fn should_solve_second_part() {
        let solver = Solver2024_16::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 0);
    }
}
