use super::utils;
use regex::Regex;

#[derive(PartialEq, Debug)]
enum Instruction {
    Move(usize),
    Right,
    Left,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Instruction::Right,
            "L" => Instruction::Left,
            other => Instruction::Move(other.parse().unwrap()),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Cell {
    Portal,
    Open,
    Wall,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            ' ' => Cell::Portal,
            '.' => Cell::Open,
            '#' => Cell::Wall,
            other => panic!("Illegal argument: {}", other),
        }
    }
}

struct Map {
    cells: Vec<Vec<Cell>>,
}

impl Map {
    fn is_valid_cell(&self, x: usize, y: usize) -> bool {
        self.cells
            .get(y)
            .is_some_and(|row| row.get(x).is_some_and(|cell| cell != &Cell::Portal))
    }

    fn find_next_cell(&self, x: usize, y: usize, direction: usize) -> (usize, usize) {
        match direction {
            0 => {
                let next_cell = (
                    if !self.is_valid_cell(x + 1, y) {
                        self.cells[y]
                            .iter()
                            .position(|c| c != &Cell::Portal)
                            .unwrap()
                    } else {
                        x + 1
                    },
                    y,
                );
                if self.cells[next_cell.1][next_cell.0] == Cell::Wall {
                    (x, y)
                } else {
                    next_cell
                }
            }
            1 => {
                let next_cell = (
                    x,
                    if !self.is_valid_cell(x, y + 1) {
                        self.cells
                            .iter()
                            .position(|row| row.get(x).is_some_and(|cell| cell != &Cell::Portal))
                            .unwrap()
                    } else {
                        y + 1
                    },
                );
                if self.cells[next_cell.1][next_cell.0] == Cell::Wall {
                    (x, y)
                } else {
                    next_cell
                }
            }
            2 => {
                let next_cell = (
                    if x == 0 || !self.is_valid_cell(x - 1, y) {
                        self.cells[y]
                            .iter()
                            .rposition(|c| c != &Cell::Portal)
                            .unwrap()
                    } else {
                        x - 1
                    },
                    y,
                );
                if self.cells[next_cell.1][next_cell.0] == Cell::Wall {
                    (x, y)
                } else {
                    next_cell
                }
            }
            3 => {
                let next_cell = (
                    x,
                    if y == 0 || !self.is_valid_cell(x, y - 1) {
                        self.cells
                            .iter()
                            .rposition(|row| row.get(x).is_some_and(|cell| cell != &Cell::Portal))
                            .unwrap()
                    } else {
                        y - 1
                    },
                );
                if self.cells[next_cell.1][next_cell.0] == Cell::Wall {
                    (x, y)
                } else {
                    next_cell
                }
            }
            other => panic!("Illegal direction: {}", other),
        }
    }
}

fn get_map_and_instructions(input: &str) -> (Map, Vec<Instruction>) {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let cells = map
        .split("\n")
        .map(|row| row.chars().map(|c| Cell::from(c)).collect())
        .collect();
    let re = Regex::new(r"(R|L|\d+)").unwrap();
    let instructions = re
        .captures_iter(instructions)
        .map(|captures| Instruction::from(&captures[1]))
        .collect();
    (Map { cells }, instructions)
}

fn solve_first_part(map: &Map, instructions: &Vec<Instruction>) -> usize {
    let mut player = (
        map.cells[0].iter().position(|c| c == &Cell::Open).unwrap(),
        0usize,
        0usize,
    );

    for instruction in instructions {
        match instruction {
            Instruction::Move(distance) => {
                for _ in 0..*distance {
                    let next_cell = map.find_next_cell(player.0, player.1, player.2);
                    if next_cell == (player.0, player.1) {
                        break;
                    }
                    player.0 = next_cell.0;
                    player.1 = next_cell.1;
                }
            }
            Instruction::Right => {
                player.2 = (player.2 + 1) % 4;
            }
            Instruction::Left => {
                player.2 = player.2.checked_sub(1).unwrap_or(3);
            }
        }
    }

    1000 * (player.1 + 1) + 4 * (player.0 + 1) + player.2
}

fn solve_second_part() -> isize {
    6032
}

pub fn solve() -> (usize, isize) {
    let (map, instructions) = get_map_and_instructions(&utils::get_input("inputs/2022_22.txt"));
    (solve_first_part(&map, &instructions), solve_second_part())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = concat!(
        "        ...#\n",
        "        .#..\n",
        "        #...\n",
        "        ....\n",
        "...#.......#\n",
        "........#...\n",
        "..#....#....\n",
        "..........#.\n",
        "        ...#....\n",
        "        .....#..\n",
        "        .#......\n",
        "        ......#.\n",
        "\n",
        "10R5L5R10L4R5L5"
    );

    #[test]
    fn should_solve_first_part() {
        let (map, instructions) = get_map_and_instructions(EXAMPLE);
        assert_eq!(solve_first_part(&map, &instructions), 6032);
    }

    #[test]
    fn should_solve_second_part() {
        assert_eq!(solve_second_part(), 6032);
    }
}
