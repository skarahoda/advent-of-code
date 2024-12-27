use super::Solver;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn to_east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn to_west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn to_north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn to_south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn to(&self, direction: char) -> Self {
        match direction {
            '>' => self.to_east(),
            '<' => self.to_west(),
            '^' => self.to_north(),
            'v' => self.to_south(),
            other => panic!("Illegal argument: {}", other),
        }
    }
}

pub struct Solver2015_03<'a> {
    directions: &'a str,
}

impl Default for Solver2015_03<'_> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2015_03<'a> {
    fn from(input: &'a str) -> Self {
        Self { directions: input }
    }
}

impl Solver<usize, usize> for Solver2015_03<'_> {
    fn solve_first_part(&self) -> usize {
        let mut locations: HashSet<Location> = HashSet::new();
        let mut location = Location::new();

        locations.insert(location);

        for direction in self.directions.chars() {
            location = location.to(direction);
            locations.insert(location);
        }
        locations.len()
    }

    fn solve_second_part(&self) -> usize {
        let mut locations: HashSet<Location> = HashSet::new();

        let mut santa_location = Location::new();
        let mut robo_santa_location = Location::new();
        let mut santa_turn = true;
        locations.insert(santa_location);
        for direction in self.directions.chars() {
            if santa_turn {
                santa_location = santa_location.to(direction);
                locations.insert(santa_location);
            } else {
                robo_santa_location = robo_santa_location.to(direction);
                locations.insert(robo_santa_location);
            }
            santa_turn = !santa_turn;
        }
        locations.len()
    }
}

#[cfg(test)]
mod first_part {
    use super::*;

    #[test]
    fn first_example() {
        let solver = Solver2015_03::from(">");
        assert_eq!(solver.solve_first_part(), 2);
    }
    #[test]
    fn second_example() {
        let solver = Solver2015_03::from("^>v<");
        assert_eq!(solver.solve_first_part(), 4);
    }
    #[test]
    fn third_example() {
        let solver = Solver2015_03::from("^v^v^v^v^v");
        assert_eq!(solver.solve_first_part(), 2);
    }
}

#[cfg(test)]
mod second_part {
    use super::*;

    #[test]
    fn first_example() {
        let solver = Solver2015_03::from("^v");
        assert_eq!(solver.solve_second_part(), 3);
    }
    #[test]
    fn second_example() {
        let solver = Solver2015_03::from("^>v<");
        assert_eq!(solver.solve_second_part(), 3);
    }
    #[test]
    fn third_example() {
        let solver = Solver2015_03::from("^v^v^v^v^v");
        assert_eq!(solver.solve_second_part(), 11);
    }
}
