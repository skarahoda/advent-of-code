use super::Solver;
mod input;
use input::INPUT;

#[derive(PartialEq, Eq, Clone)]
enum Cell {
    Empty,
    Wall,
    Box,
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Coordinate(usize, usize);

impl Coordinate {
    fn move_by(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.1 -= 1,
            Direction::Down => self.1 += 1,
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
        }
    }
}

#[derive(Clone)]
pub struct Solver2024_15 {
    map: Vec<Vec<Cell>>,
    player: Coordinate,
    directions: Vec<Direction>,
}

impl Default for Solver2024_15 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

impl Solver2024_15 {
    fn find_first_wall_or_empty_cell(
        &self,
        coordinate: &Coordinate,
        direction: &Direction,
    ) -> (Coordinate, bool) {
        let mut coordinate = coordinate.clone();
        loop {
            coordinate.move_by(direction);

            let cell = &self.map[coordinate.1][coordinate.0];
            match cell {
                Cell::Wall => return (coordinate.clone(), false),
                Cell::Empty => return (coordinate.clone(), true),
                _ => {}
            }
        }
    }

    fn set_boxes(&mut self, first_empty_cell: &Coordinate) {
        let min_x = first_empty_cell.0.min(self.player.0);
        let min_y = first_empty_cell.1.min(self.player.1);
        let max_x = first_empty_cell.0.max(self.player.0);
        let max_y = first_empty_cell.1.max(self.player.1);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                self.map[y][x] = Cell::Box;
            }
        }
    }

    fn move_player(&mut self) {
        while let Some(direction) = self.directions.pop() {
            let (first_cell, is_empty) =
                self.find_first_wall_or_empty_cell(&self.player, &direction);
            if is_empty {
                self.player.move_by(&direction);
                self.set_boxes(&first_cell);
                self.map[self.player.1][self.player.0] = Cell::Empty;
            }
        }
    }

    fn calculate_score(&self) -> usize {
        self.map.iter().enumerate().fold(0, |acc, (y, row)| {
            row.iter().enumerate().fold(acc, |acc, (x, cell)| {
                if *cell == Cell::Box {
                    acc + (x + 100usize * y)
                } else {
                    acc
                }
            })
        })
    }
}

impl From<&str> for Solver2024_15 {
    fn from(input: &str) -> Self {
        let parts = input.split("\n\n").collect::<Vec<_>>();
        let map = parts[0]
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' | '@' => Cell::Empty,
                        '#' => Cell::Wall,
                        'O' => Cell::Box,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        let directions = parts[1]
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Direction::Up),
                '>' => Some(Direction::Right),
                'v' => Some(Direction::Down),
                '<' => Some(Direction::Left),
                '\n' => None,
                _ => unreachable!(),
            })
            .rev()
            .collect();
        let player = parts[0]
            .lines()
            .enumerate()
            .find_map(|(y, line)| Some(Coordinate(line.find(|c| c == '@')?, y)))
            .unwrap();
        Self {
            map,
            player,
            directions,
        }
    }
}

impl Solver<usize, u64> for Solver2024_15 {
    fn solve_first_part(&self) -> usize {
        let mut mutated = self.clone();
        mutated.move_player();
        mutated.calculate_score()
    }

    fn solve_second_part(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<\
";

    const BIG_EXAMPLE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\
";

    #[test]
    fn should_solve_first_part() {
        let solver = Solver2024_15::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 2028);
    }

    #[test]
    fn should_solve_first_part_big() {
        let solver = Solver2024_15::from(BIG_EXAMPLE);
        assert_eq!(solver.solve_first_part(), 10092);
    }

    #[test]
    fn should_solve_second_part() {
        let solver = Solver2024_15::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 0);
    }
}
