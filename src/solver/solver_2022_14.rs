use std::cmp::{max, min};
use super::utils;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Material {
    Sand,
    Rock,
    Air
}

type Grid = [[Material;700];200];

fn get_grid(input: &str) -> Grid {
    let mut result = [[Material::Air;700];200];
    let paths = input.split("\n");
    for path in paths {
        let edges: Vec<(usize, usize)> = path.split(" -> ")
            .map(|edge| {
                let coordinates: Vec<usize> = edge.split(",").map(|val| val.parse().unwrap()).collect();
                (coordinates[0], coordinates[1])
            }).collect();
        for i in 1..edges.len() {
            let first_edge = edges[i-1];
            let second_edge = edges[i];
            if first_edge.0 == second_edge.0 {
                let x = first_edge.0;
                for y in min(first_edge.1, second_edge.1)..=max(first_edge.1, second_edge.1) {
                    result[y][x] = Material::Rock
                }
            } else {
                let y = first_edge.1;
                for x in min(first_edge.0, second_edge.0)..=max(first_edge.0, second_edge.0) {
                    result[y][x] = Material::Rock
                }
            }
        }
    }
    result
}

#[derive(Debug)]
enum DropSandError {
    DestinationVoid
}

fn drop_sand(grid: &Grid) -> Result<(usize, usize), DropSandError> {
    let mut current_location = (500usize, 0usize);
    while current_location.0 > 0 && current_location.0 < 699 && current_location.1 < 199  {
        if grid[current_location.1 + 1][current_location.0] == Material::Air {
            current_location.1 += 1;
        } else if grid[current_location.1 + 1][current_location.0 - 1] == Material::Air {
            current_location.0 -= 1;
            current_location.1 += 1;
        } else if grid[current_location.1 + 1][current_location.0 + 1] == Material::Air {
            current_location.0 += 1;
            current_location.1 += 1;
        } else {
            return Ok(current_location);
        }
    }
    Err(DropSandError::DestinationVoid)
}

fn solve_first_part(grid: &mut Grid) -> u32 {
    let mut result = 0;
    while let Ok(location) = drop_sand(grid) {
        grid[location.1][location.0] = Material::Sand;
        result += 1;
    }
    result
}

fn solve_second_part(grid: &mut Grid) -> u32 {
    let (max_y, _) = grid.iter().enumerate().rfind(|(_, row)| row.contains(&Material::Rock)).unwrap();
    grid[max_y+2] = [Material::Rock; 700];

    let mut result = 0;
    loop {
        let (x, y) = drop_sand(grid).unwrap();
        grid[y][x] = Material::Sand;
        result += 1;
        if y == 0 {
            return result;
        }
    }
}

pub fn solve() -> (u32, u32) {
    let input = utils::get_input("inputs/2022_14.txt");
    (
        solve_first_part(&mut get_grid(&input)),
        solve_second_part(&mut get_grid(&input)),
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str =
        "\
        498,4 -> 498,6 -> 496,6\n\
        503,4 -> 502,4 -> 502,9 -> 494,9"
    ;

    #[test]
    fn should_solve_first_part_example() {
        let mut grid = get_grid(EXAMPLE);
        assert_eq!(
            solve_first_part(&mut grid),
            24
        );
    }
    #[test]
    fn should_solve_second_part_example() {
        let mut grid = get_grid(EXAMPLE);
        assert_eq!(
            solve_second_part(&mut grid),
            93
        );
    }
}

