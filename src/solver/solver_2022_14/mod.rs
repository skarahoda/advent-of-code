use super::Solver;
use std::cmp::{max, min};

#[derive(Copy, Clone, PartialEq)]
enum Material {
    Sand,
    Rock,
    Air,
}

#[derive(Debug)]
enum DropSandError {
    DestinationVoid,
}

#[derive(Clone)]
pub struct Solver2022_14 {
    grid: [[Material; 700]; 200],
}

impl Default for Solver2022_14 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2022_14 {
    fn from(input: &str) -> Self {
        let mut grid = [[Material::Air; 700]; 200];
        let paths = input.split("\n");
        for path in paths {
            let edges: Vec<(usize, usize)> = path
                .split(" -> ")
                .map(|edge| {
                    let coordinates: Vec<usize> =
                        edge.split(",").map(|val| val.parse().unwrap()).collect();
                    (coordinates[0], coordinates[1])
                })
                .collect();
            for i in 1..edges.len() {
                let first_edge = edges[i - 1];
                let second_edge = edges[i];
                if first_edge.0 == second_edge.0 {
                    let x = first_edge.0;
                    for y in min(first_edge.1, second_edge.1)..=max(first_edge.1, second_edge.1) {
                        grid[y][x] = Material::Rock
                    }
                } else {
                    let y = first_edge.1;
                    for x in min(first_edge.0, second_edge.0)..=max(first_edge.0, second_edge.0) {
                        grid[y][x] = Material::Rock
                    }
                }
            }
        }
        Self { grid }
    }
}

impl Solver2022_14 {
    fn drop_sand(&self) -> Result<(usize, usize), DropSandError> {
        let mut current_location = (500usize, 0usize);
        while current_location.0 > 0 && current_location.0 < 699 && current_location.1 < 199 {
            if self.grid[current_location.1 + 1][current_location.0] == Material::Air {
                current_location.1 += 1;
            } else if self.grid[current_location.1 + 1][current_location.0 - 1] == Material::Air {
                current_location.0 -= 1;
                current_location.1 += 1;
            } else if self.grid[current_location.1 + 1][current_location.0 + 1] == Material::Air {
                current_location.0 += 1;
                current_location.1 += 1;
            } else {
                return Ok(current_location);
            }
        }
        Err(DropSandError::DestinationVoid)
    }
}

impl Solver<u32, u32> for Solver2022_14 {
    fn solve_first_part(&self) -> u32 {
        let mut mutated = self.clone();
        let mut result = 0;
        while let Ok(location) = mutated.drop_sand() {
            mutated.grid[location.1][location.0] = Material::Sand;
            result += 1;
        }
        result
    }

    fn solve_second_part(&self) -> u32 {
        let mut mutated = self.clone();
        let (max_y, _) = mutated
            .grid
            .iter()
            .enumerate()
            .rfind(|(_, row)| row.contains(&Material::Rock))
            .unwrap();
        mutated.grid[max_y + 2] = [Material::Rock; 700];

        let mut result = 0;
        loop {
            let (x, y) = mutated.drop_sand().unwrap();
            mutated.grid[y][x] = Material::Sand;
            result += 1;
            if y == 0 {
                return result;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
        498,4 -> 498,6 -> 496,6\n\
        503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn should_solve_first_part() {
        let solver = Solver2022_14::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 24);
    }

    #[test]
    fn should_solve_second_part() {
        let solver = Solver2022_14::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 93);
    }
}
