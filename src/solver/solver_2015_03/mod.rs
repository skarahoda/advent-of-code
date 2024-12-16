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

fn solve_first_part(directions: &str) -> usize {
    let mut locations: HashSet<Location> = HashSet::new();
    let mut location = Location::new();

    locations.insert(location);

    for direction in directions.chars() {
        location = location.to(direction);
        locations.insert(location);
    }
    locations.len()
}

fn solve_second_part(directions: &str) -> usize {
    let mut locations: HashSet<Location> = HashSet::new();

    let mut santa_location = Location::new();
    let mut robo_santa_location = Location::new();
    let mut santa_turn = true;
    locations.insert(santa_location);
    for direction in directions.chars() {
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

pub fn solve() -> (usize, usize) {
    (
        solve_first_part(include_str!("input.txt")),
        solve_second_part(include_str!("input.txt")),
    )
}

#[cfg(test)]
mod first_part {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(solve_first_part(">"), 2);
    }
    #[test]
    fn second_example() {
        assert_eq!(solve_first_part("^>v<"), 4);
    }
    #[test]
    fn third_example() {
        assert_eq!(solve_first_part("^v^v^v^v^v"), 2);
    }
}

#[cfg(test)]
mod second_part {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(solve_second_part("^v"), 3);
    }
    #[test]
    fn second_example() {
        assert_eq!(solve_second_part("^>v<"), 3);
    }
    #[test]
    fn third_example() {
        assert_eq!(solve_second_part("^v^v^v^v^v"), 11);
    }
}
