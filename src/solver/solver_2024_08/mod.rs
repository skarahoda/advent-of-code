use std::collections::{HashMap, HashSet};
mod input;
use input::INPUT;

fn parse_inputs(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_antennas(input: &Vec<Vec<char>>) -> HashMap<char, HashSet<(usize, usize)>> {
    let mut result = HashMap::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != '.' {
                result.entry(*c).or_insert_with(HashSet::new).insert((x, y));
            }
        }
    }
    result
}

fn get_anti_node(map: &Vec<Vec<char>>, first_position: &(usize, usize), second_position: &(usize, usize), multiplier: usize) -> Option<(usize, usize)> {
    let min_x = first_position.0.min(second_position.0);
    let max_x = first_position.0.max(second_position.0);
    let min_y = first_position.1.min(second_position.1);
    let max_y = first_position.1.max(second_position.1);
    let distance_x = max_x.checked_sub(min_x)?;
    let distance_y = max_y.checked_sub(min_y)?;
    let x = if first_position.0 < second_position.0 { min_x.checked_sub(distance_x.checked_mul(multiplier)?)? } else { max_x.checked_add(distance_x.checked_mul(multiplier)?)? };
    let y = if first_position.1 < second_position.1 { min_y.checked_sub(distance_y.checked_mul(multiplier)?)? } else { max_y.checked_add(distance_y.checked_mul(multiplier)?)? };
    map.get(y)?.get(x).map(|_| (x, y))
}

fn solve_first_part(map: &Vec<Vec<char>>) -> usize {
    let antennas = get_antennas(map);
    let mut anti_nodes = HashSet::new();
    for  positions in antennas.values() {
        for (i, first_position) in positions.iter().enumerate() {
            for second_position in positions.iter().skip(i + 1) {
                if let Some(anti_node) = get_anti_node(map, first_position, second_position, 1) {
                    anti_nodes.insert(anti_node);
                }
                if let Some(anti_node) = get_anti_node(map, second_position, first_position, 1) {
                    anti_nodes.insert(anti_node);
                }
            }
        }
    }
    anti_nodes.len()
}


fn solve_second_part(map: &Vec<Vec<char>>) -> usize {
    let antennas = get_antennas(map);
    let mut anti_nodes = HashSet::new();
    for  positions in antennas.values() {
        for (i, first_position) in positions.iter().enumerate() {
            anti_nodes.insert(*first_position);
            for second_position in positions.iter().skip(i + 1) {
                let mut multiplier = 1usize;
                while let Some(anti_node) = get_anti_node(map, first_position, second_position, multiplier) {
                    anti_nodes.insert(anti_node);
                    multiplier += 1;
                }

                multiplier = 1usize;
                while let Some(anti_node) = get_anti_node(map, second_position, first_position, multiplier) {
                    anti_nodes.insert(anti_node);
                    multiplier += 1;
                }
            }
        }
    }
    anti_nodes.len()
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
    static EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    #[test]
    fn solve_first_part() {
        let map = super::parse_inputs(EXAMPLE);
        assert_eq!(
            super::solve_first_part(&map),
            14
        );
    }

    #[test]
    fn solve_second_part() {
        let map = super::parse_inputs(EXAMPLE);
        assert_eq!(
            super::solve_second_part(&map),
            34
        );
    }
}
