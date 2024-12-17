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

impl From<(&Coordinate, &Coordinate)> for Direction {
    fn from((a, b): (&Coordinate, &Coordinate)) -> Self {
        if a.0 == b.0 && a.1 > b.1 {
            Direction::Up
        } else if a.0 == b.0 && a.1 < b.1 {
            Direction::Down
        } else if a.1 == b.1 && a.0 > b.0 {
            Direction::Left
        } else if a.1 == b.1 && a.0 < b.0 {
            Direction::Right
        } else {
            unreachable!()
        }
    }
}

fn get_entries(
    coordinate: &Coordinate,
    map: &HashMap<(Coordinate, Direction), (usize, Vec<Coordinate>)>,
) -> Vec<(Direction, usize)> {
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    .filter_map(|direction| {
        map.get(&(coordinate.clone(), direction.clone()))
            .map(|(score, _)| (direction.clone(), score.clone()))
    })
    .collect()
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

    fn create_map(&self) -> HashMap<(Coordinate, Direction), (usize, Vec<Coordinate>)> {
        let mut processed_cells: HashSet<(Coordinate, Direction)> = HashSet::new();
        let mut map: HashMap<(Coordinate, Direction), (usize, Vec<Coordinate>)> = HashMap::new();

        map.insert((self.start.clone(), Direction::Up), (1000, vec![]));
        map.insert((self.start.clone(), Direction::Down), (1000, vec![]));
        map.insert((self.start.clone(), Direction::Left), (2000, vec![]));
        map.insert((self.start.clone(), Direction::Right), (0, vec![]));

        while processed_cells.len() < map.len() {
            let ((min_cell, direction), (score, _)) = map
                .iter()
                .filter(|(key, _)| !processed_cells.contains(key))
                .min_by_key(|(_, (value, _))| *value)
                .unwrap();
            let min_cell = min_cell.clone();
            let direction = direction.clone();
            let score = score.clone();

            processed_cells.insert((min_cell.clone(), direction.clone()));

            let next_cell = &min_cell.next(&direction);
            if self.is_valid(next_cell) {
                let next_score = score + 1;
                map.entry((next_cell.clone(), direction.clone()))
                    .and_modify(|(value, previous_cells)| {
                        if *value == next_score {
                            previous_cells.push(min_cell.clone());
                        } else if *value > next_score {
                            *value = next_score;
                            *previous_cells = vec![min_cell.clone()];
                        }
                    })
                    .or_insert((next_score, vec![min_cell.clone()]));
                let next_score = next_score + 1000;
                map.entry((next_cell.clone(), direction.rotate_clockwise()))
                    .and_modify(|(value, previous_cells)| {
                        if *value == next_score {
                            previous_cells.push(min_cell.clone());
                        } else if *value > next_score {
                            *value = next_score;
                            *previous_cells = vec![min_cell.clone()];
                        }
                    })
                    .or_insert((next_score, vec![min_cell.clone()]));
                map.entry((next_cell.clone(), direction.rotate_counter_clockwise()))
                    .and_modify(|(value, previous_cells)| {
                        if *value == next_score {
                            previous_cells.push(min_cell.clone());
                        } else if *value > next_score {
                            *value = next_score;
                            *previous_cells = vec![min_cell.clone()];
                        }
                    })
                    .or_insert((next_score, vec![min_cell.clone()]));
            }
        }
        map
    }

    fn find_shortest_path(&self) -> Option<usize> {
        let map = self.create_map();
        get_entries(&self.end, &map)
            .iter()
            .map(|(_, score)| score)
            .min()
            .cloned()
    }

    fn find_best_seats(&self) -> Option<usize> {
        let map = self.create_map();
        let mut best_seats = HashSet::new();
        best_seats.insert(self.end.clone());
        let entries = get_entries(&self.end, &map);
        let min_score = entries.iter().map(|(_, score)| score).min().cloned()?;
        let mut queue: Vec<(Coordinate, Direction)> = entries
            .iter()
            .filter(|(_, score)| *score == min_score)
            .map(|(direction, _)| (self.end.clone(), direction.clone()))
            .collect();

        while let Some((cell, direction)) = queue.pop() {
            best_seats.insert(cell.clone());
            let (_, previous_cells) = map.get(&(cell.clone(), direction.clone()))?;
            queue.extend(
                previous_cells
                    .iter()
                    .map(|prev_cell| (prev_cell.clone(), (prev_cell, &cell).into())),
            );
        }

        Some(best_seats.len())
    }
}

impl Solver<usize, usize> for Solver2024_16 {
    fn solve_first_part(&self) -> usize {
        self.find_shortest_path().unwrap()
    }

    fn solve_second_part(&self) -> usize {
        self.find_best_seats().unwrap()
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
        assert_eq!(solver.solve_second_part(), 45);
    }
}
