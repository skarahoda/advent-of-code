mod input;
use input::INPUT;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Cell {
    Rock,
    Air,
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Air => '.',
            Cell::Rock => '#',
        }
    }
}

struct Chamber {
    height: usize,
    cells: Vec<[Cell; 7]>,
}

impl Chamber {
    fn new() -> Self {
        Self {
            height: 0,
            cells: Vec::new(),
        }
    }

    fn get_reduced_height(&self, y: usize) -> usize {
        y - (self.height - self.cells.len())
    }

    fn is_blocked(&self, coordinates: &Vec<(Option<usize>, Option<usize>)>) -> bool {
        coordinates.iter().any(|&(x, y)| match (x, y) {
            (None, _) => true,
            (_, None) => true,
            (Some(x), Some(y)) => {
                y < self.height && self.cells[self.get_reduced_height(y)][x] == Cell::Rock
            }
        })
    }

    fn set_rocks(&mut self, coordinates: &Vec<(usize, usize)>) {
        for &(x, y) in coordinates {
            while y >= self.height {
                self.height += 1;
                self.cells.push([Cell::Air; 7]);
            }
            let reduced_height = self.get_reduced_height(y);
            self.cells[reduced_height][x] = Cell::Rock;
        }
    }

    fn get_surface(&self) -> Self {
        let mut result = Self::new();
        let mut rows = self.cells.iter().rev();
        while result
            .cells
            .last()
            .unwrap_or(&[Cell::Air; 7])
            .iter()
            .any(|&c| c != Cell::Rock)
        {
            let mut new_row = [Cell::Rock; 7];
            let previous_row = result.cells.last().unwrap_or(&[Cell::Air; 7]);
            if let Some(row) = rows.next() {
                let mut air_queue: Vec<usize> = previous_row
                    .iter()
                    .enumerate()
                    .filter(|&(i, cell)| *cell == Cell::Air && row[i] == Cell::Air)
                    .map(|(i, _)| i)
                    .collect();
                let mut air_cells: HashSet<usize> = air_queue.iter().map(|&v| v).collect();
                while let Some(i) = air_queue.pop() {
                    if i > 0 && !air_cells.contains(&(i - 1)) && row[i - 1] == Cell::Air {
                        air_cells.insert(i - 1);
                        air_queue.push(i - 1);
                    }
                    if i < 6 && !air_cells.contains(&(i + 1)) && row[i + 1] == Cell::Air {
                        air_cells.insert(i + 1);
                        air_queue.push(i + 1);
                    }
                }
                for air_cell in air_cells {
                    new_row[air_cell] = Cell::Air;
                }
            }
            result.cells.push(new_row);
            result.height += 1;
        }
        result
    }
}

impl ToString for Chamber {
    fn to_string(&self) -> String {
        self.cells.iter().fold(String::new(), |rest, row| {
            let row: String = row.map(|cell| char::from(cell)).iter().collect();
            row + "\n" + rest.as_str()
        })
    }
}

struct Rock {
    shape: usize,
    x: usize,
    y: usize,
}

impl Rock {
    fn new(shape: usize, x: usize, y: usize) -> Self {
        Self { shape, x, y }
    }
    fn get_cells(&self) -> Vec<(usize, usize)> {
        match self.shape {
            0 => vec![
                (self.x, self.y),
                (self.x + 1, self.y),
                (self.x + 2, self.y),
                (self.x + 3, self.y),
            ], // horizontal line shape
            1 => vec![
                (self.x, self.y + 1),
                (self.x + 1, self.y),
                (self.x + 1, self.y + 1),
                (self.x + 1, self.y + 2),
                (self.x + 2, self.y + 1),
            ], // plus shape
            2 => vec![
                (self.x, self.y),
                (self.x + 1, self.y),
                (self.x + 2, self.y),
                (self.x + 2, self.y + 1),
                (self.x + 2, self.y + 2),
            ], // boomerang shape
            3 => vec![
                (self.x, self.y),
                (self.x, self.y + 1),
                (self.x, self.y + 2),
                (self.x, self.y + 3),
            ], // vertical line shape
            4 => vec![
                (self.x, self.y),
                (self.x, self.y + 1),
                (self.x + 1, self.y),
                (self.x + 1, self.y + 1),
            ], //square shape
            other => panic!("Illegal argument: {}", other),
        }
    }

    fn get_lower_cells(&self) -> Vec<(Option<usize>, Option<usize>)> {
        let mut map: HashMap<usize, Option<usize>> = HashMap::new();
        for (x, y) in self.get_cells() {
            let entry = map.get(&x);
            if entry.is_none() || (entry.unwrap().is_some() && entry.unwrap().unwrap() >= y) {
                map.insert(x, if y == 0 { None } else { Some(y - 1) });
            }
        }
        map.iter().map(|(&x, &y)| (Some(x), y)).collect()
    }

    fn get_right_cells(&self) -> Vec<(Option<usize>, Option<usize>)> {
        let mut map: HashMap<usize, Option<usize>> = HashMap::new();
        for (x, y) in self.get_cells() {
            let entry = map.get(&y);
            if entry.is_none() || (entry.unwrap().is_some() && entry.unwrap().unwrap() <= x) {
                map.insert(y, if x == 6 { None } else { Some(x + 1) });
            }
        }
        map.iter().map(|(&y, &x)| (x, Some(y))).collect()
    }

    fn get_left_cells(&self) -> Vec<(Option<usize>, Option<usize>)> {
        let mut map: HashMap<usize, Option<usize>> = HashMap::new();
        for (x, y) in self.get_cells() {
            let entry = map.get(&y);
            if entry.is_none() || (entry.unwrap().is_some() && entry.unwrap().unwrap() >= x) {
                map.insert(y, if x == 0 { None } else { Some(x - 1) });
            }
        }
        map.iter().map(|(&y, &x)| (x, Some(y))).collect()
    }

    fn can_go_right(&self, chamber: &Chamber) -> bool {
        !chamber.is_blocked(&self.get_right_cells())
    }

    fn can_go_left(&self, chamber: &Chamber) -> bool {
        !chamber.is_blocked(&self.get_left_cells())
    }

    fn can_go_down(&self, chamber: &Chamber) -> bool {
        !chamber.is_blocked(&self.get_lower_cells())
    }

    fn go_right(&mut self) {
        self.x += 1;
    }

    fn go_left(&mut self) {
        self.x -= 1;
    }

    fn go_down(&mut self) {
        self.y -= 1;
    }

    fn add_to_chamber(&self, chamber: &mut Chamber) {
        chamber.set_rocks(&self.get_cells());
    }
}

#[derive(Debug, PartialEq)]
enum Move {
    Left,
    Right,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '<' => Move::Left,
            '>' => Move::Right,
            other => panic!("Illegal argument: {}", other),
        }
    }
}

fn get_moves(input: &str) -> Vec<Move> {
    input.chars().map(|c| Move::from(c)).collect()
}

fn solve_puzzle(moves: &Vec<Move>, number_of_rocks: usize) -> usize {
    let mut chamber: Chamber = Chamber::new();
    let mut move_count = 0;
    let mut cycle_finder: HashMap<(usize, usize, String), (usize, usize)> = HashMap::new();
    let mut i = 0;
    let mut found_cycle = false;
    while i < number_of_rocks {
        let rock_index = i % 5;
        if !found_cycle {
            let move_index = move_count % moves.len();
            let surface = chamber.get_surface();
            if let Some((previous_height, previous_rock_count)) =
                cycle_finder.get(&(rock_index, move_index, surface.to_string()))
            {
                let cycle_rock_count = i - previous_rock_count;
                let rest_rocks = number_of_rocks - previous_rock_count;
                let cycle_height = chamber.height - previous_height;
                let cycle_count = rest_rocks / cycle_rock_count;
                chamber.height = previous_height + (cycle_count * cycle_height);
                i = previous_rock_count + (cycle_count * cycle_rock_count);
                found_cycle = true;
            } else {
                cycle_finder.insert(
                    (rock_index, move_index, surface.to_string()),
                    (chamber.height, i),
                );
            }
        }

        let mut rock = Rock::new(rock_index, 2, chamber.height + 3);
        loop {
            match moves[move_count % moves.len()] {
                Move::Left => {
                    if rock.can_go_left(&chamber) {
                        rock.go_left();
                    }
                }
                Move::Right => {
                    if rock.can_go_right(&chamber) {
                        rock.go_right();
                    }
                }
            }
            move_count += 1;

            if rock.can_go_down(&chamber) {
                rock.go_down();
            } else {
                rock.add_to_chamber(&mut chamber);
                break;
            }
        }
        i += 1;
    }
    chamber.height
}

fn solve_first_part(moves: &Vec<Move>) -> usize {
    solve_puzzle(moves, 2022)
}

fn solve_second_part(moves: &Vec<Move>) -> usize {
    solve_puzzle(moves, 1000000000000)
}

pub fn solve() -> (usize, usize) {
    let moves = get_moves(INPUT);
    (solve_first_part(&moves), solve_second_part(&moves))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn should_get_moves() {
        assert_eq!(
            get_moves(EXAMPLE),
            vec![
                Move::Right,
                Move::Right,
                Move::Right,
                Move::Left,
                Move::Left,
                Move::Right,
                Move::Left,
                Move::Right,
                Move::Right,
                Move::Left,
                Move::Left,
                Move::Left,
                Move::Right,
                Move::Right,
                Move::Left,
                Move::Right,
                Move::Right,
                Move::Right,
                Move::Left,
                Move::Left,
                Move::Left,
                Move::Right,
                Move::Right,
                Move::Right,
                Move::Left,
                Move::Left,
                Move::Left,
                Move::Right,
                Move::Left,
                Move::Left,
                Move::Left,
                Move::Right,
                Move::Right,
                Move::Left,
                Move::Right,
                Move::Right,
                Move::Left,
                Move::Left,
                Move::Right,
                Move::Right,
            ]
        );
    }

    #[test]
    fn should_solve_first_part() {
        assert_eq!(solve_first_part(&get_moves(EXAMPLE)), 3068);
    }

    #[test]
    fn should_solve_second_part() {
        assert_eq!(solve_second_part(&get_moves(EXAMPLE)), 1514285714288);
    }
}
