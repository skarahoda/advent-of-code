use std::collections::HashSet;
mod input;
use super::Solver;
use input::INPUT;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn rotate_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

#[derive(Clone)]
pub struct Solver202406 {
    map: Vec<Vec<char>>,
}

impl Solver202406 {
    fn get_next_position(
        &self,
        position: (usize, usize),
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::Up => Some((position.0, position.1.checked_sub(1)?)),
            Direction::Down => {
                if self.map.len() == position.1 + 1 {
                    None
                } else {
                    Some((position.0, position.1.checked_add(1)?))
                }
            }
            Direction::Left => Some((position.0.checked_sub(1)?, position.1)),
            Direction::Right => {
                if self.map[position.1].len() == position.0 + 1 {
                    None
                } else {
                    Some((position.0.checked_add(1)?, position.1))
                }
            }
        }
    }

    fn get_char(&self, position: (usize, usize)) -> Option<&char> {
        self.map.get(position.1)?.get(position.0)
    }

    fn get_path(&self, initial_position: (usize, usize)) -> Option<HashSet<(usize, usize)>> {
        let mut direction = Direction::Up;
        let mut visited = HashSet::new();
        visited.insert((initial_position.0, initial_position.1, direction));
        let mut current_position = initial_position;
        while let Some(next_position) = self.get_next_position(current_position, &direction) {
            match self.get_char(next_position) {
                Some('^') | Some('.') => {
                    // move forward
                    current_position = next_position;
                }
                Some('#') => {
                    direction = rotate_right(&direction);
                }
                val => {
                    panic!("Unexpected value: {:?}", val);
                }
            }
            if visited.contains(&(current_position.0, current_position.1, direction)) {
                return None;
            }
            visited.insert((current_position.0, current_position.1, direction));
        }
        Some(visited.iter().map(|(x, y, _)| (*x, *y)).collect())
    }
    fn find_initial_position(&self) -> (usize, usize) {
        for (y, line) in self.map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == '^' {
                    return (x, y);
                }
            }
        }
        panic!("No initial position found");
    }
}

impl From<&str> for Solver202406 {
    fn from(value: &str) -> Self {
        Self {
            map: value.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

impl Default for Solver202406 {
    fn default() -> Self {
        INPUT.into()
    }
}

impl Solver<usize, usize> for Solver202406 {
    fn solve_first_part(&self) -> usize {
        self.get_path(self.find_initial_position()).unwrap().len()
    }

    fn solve_second_part(&self) -> usize {
        let path = self.get_path(self.find_initial_position()).unwrap();
        let mut result = 0;
        for (x, y) in path.iter() {
            if self.map[*y][*x] != '.' {
                continue;
            }
            let mut modified_map = self.clone();
            modified_map.map[*y][*x] = '#';
            if modified_map
                .get_path(modified_map.find_initial_position())
                .is_none()
            {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...:";
    #[test]
    fn solve_first_part() {
        assert_eq!(Solver202406::from(EXAMPLE).solve_first_part(), 41);
    }

    #[test]
    fn solve_second_part() {
        assert_eq!(Solver202406::from(EXAMPLE).solve_second_part(), 6);
    }
}
