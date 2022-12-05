use std::collections::HashSet;
use super::utils;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    fn to_east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y
        }
    }

    fn to_west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y
        }
    }

    fn to_north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1
        }
    }

    fn to_south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1
        }
    }

    fn to(&self, direction: char) -> Self {
        match direction {
            '>' => self.to_east(),
            '<' => self.to_west(),
            '^' => self.to_north(),
            'v' => self.to_south(),
            other => panic!("Illegal argument: {}", other)
        }
    }
}


pub fn solve_first_part() -> usize {
    let directions = utils::get_input("inputs/2015_03.txt");

    let mut locations: HashSet<Location> = HashSet::new();
    let mut location = Location::new();

    locations.insert(location);

    for direction in directions.chars() {
        location = location.to(direction);
        locations.insert(location);
    }
    locations.len()
}

pub fn solve_second_part() -> usize {
    let directions = utils::get_input("inputs/2015_03.txt");

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
