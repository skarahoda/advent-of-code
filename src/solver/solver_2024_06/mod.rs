use std::collections::HashSet;
mod input;
use input::INPUT;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_inputs(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}


fn find_initial_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '^' {
                return (x, y);
            }
        }
    }
    panic!("No initial position found");
}

fn get_next_position(map: &Vec<Vec<char>>, position: (usize, usize), direction: &Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => Some((position.0, position.1.checked_sub(1)?)),
        Direction::Down => if map.len() == position.1 + 1 { None } else { Some((position.0, position.1.checked_add(1)?)) },
        Direction::Left => Some((position.0.checked_sub(1)?, position.1)),
        Direction::Right => if map[position.1].len() == position.0 + 1 { None } else { Some((position.0.checked_add(1)?, position.1)) },
    }
}

fn rotate_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_char(map: &Vec<Vec<char>>, position: (usize, usize)) -> Option<&char> {
    map.get(position.1)?.get(position.0)
}

fn get_path(map: &Vec<Vec<char>>, initial_position: (usize, usize)) -> Option<HashSet<(usize, usize)>> {
    let mut direction = Direction::Up;
    let mut visited = HashSet::new();
    visited.insert((initial_position.0, initial_position.1, direction));
    let mut current_position = initial_position;
    while let Some(next_position) = get_next_position(map, current_position, &direction) {
        match get_char(map, next_position) {
            Some('^') | Some('.') => {
                // move forward
                current_position = next_position;
            }
            Some('#') => {
                direction = rotate_right(&direction);
            }
            val => { panic!("Unexpected value: {:?}", val); }
        }
        if visited.contains(&(current_position.0, current_position.1, direction)) {
            return None;
        }
        visited.insert((current_position.0, current_position.1, direction));
    }
    Some(visited.iter().map(|(x, y, _)| (*x, *y)).collect())
}
fn solve_first_part(map: &Vec<Vec<char>>) -> usize {
    get_path(map, find_initial_position(map)).unwrap().len()
}


fn solve_second_part(map: &Vec<Vec<char>>) -> usize {
    let path = get_path(&map, find_initial_position(&map)).unwrap();
    let mut result = 0;
    for (x, y) in path.iter() {
        if map[*y][*x] != '.' {
            continue;
        }
        let mut modified_map = map.clone();
        modified_map[*y][*x] = '#';
        if get_path(&modified_map, find_initial_position(&modified_map)).is_none() {
            result += 1;
        }
    }
    result
}

pub fn solve() -> (usize, usize) {
    let map = parse_inputs(INPUT);
    (
        solve_first_part(&map),
        solve_second_part(&map),
    )
}

#[cfg(test)]
mod tests {
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
        let map = super::parse_inputs(EXAMPLE);
        assert_eq!(
            super::solve_first_part(&map),
            41
        );
    }

    #[test]
    fn solve_second_part() {
        let map = super::parse_inputs(EXAMPLE);
        assert_eq!(
            super::solve_second_part(&map),
            6
        );
    }
}
