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
    fn next(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self(self.0, self.1 - 1),
            Direction::Down => Self(self.0, self.1 + 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Right => Self(self.0 + 1, self.1),
        }
    }
}

#[derive(Clone)]
pub struct Game {
    map: Vec<Vec<Cell>>,
    player: Coordinate,
    directions: Vec<Direction>,
}

impl From<&str> for Game {
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

impl Game {
    fn find_boxes_on_the_way(&mut self, direction: &Direction) -> Option<Vec<Coordinate>> {
        let mut boxes = Vec::new();
        let mut queue = vec![self.player.next(direction)];
        while let Some(current_coordinate) = queue.pop() {
            let cell = &self.map[current_coordinate.1][current_coordinate.0];
            match cell {
                Cell::Wall => return None,
                Cell::Empty => {}
                Cell::Box => {
                    boxes.push(current_coordinate.clone());
                    queue.push(current_coordinate.next(direction));
                }
            }
        }
        Some(boxes)
    }

    fn move_boxes(&mut self, boxes: &Vec<Coordinate>, direction: &Direction) {
        for box_coordinate in boxes.iter().rev() {
            self.map[box_coordinate.1][box_coordinate.0] = Cell::Empty;
            let next_coordinate = box_coordinate.next(direction);
            self.map[next_coordinate.1][next_coordinate.0] = Cell::Box;
        }
    }

    fn move_player(&mut self) {
        while let Some(direction) = self.directions.pop() {
            let boxes = self.find_boxes_on_the_way(&direction);
            if let Some(boxes) = boxes {
                self.move_boxes(&boxes, &direction);
                self.player.move_by(&direction);
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

#[derive(Clone)]
pub struct Solver2024_15 {
    part1: Game,
}

impl Default for Solver2024_15 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

impl From<&str> for Solver2024_15 {
    fn from(input: &str) -> Self {
        let part1 = Game::from(input);
        Self { part1 }
    }
}

impl Solver<usize, u64> for Solver2024_15 {
    fn solve_first_part(&self) -> usize {
        let mut mutated = self.clone();
        mutated.part1.move_player();
        mutated.part1.calculate_score()
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
