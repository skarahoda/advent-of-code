use super::Solver;
use std::cmp::Ordering;

const BOARD_SIZE: usize = 1000;
#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Rope {
    knots: Vec<(usize, usize)>,
}

impl Rope {
    fn new(length: usize) -> Self {
        let middle = (BOARD_SIZE / 2) - 1;
        Self {
            knots: vec![(middle, middle); length],
        }
    }

    fn distance(&self, i: usize, j: usize) -> usize {
        let max_x = std::cmp::max(self.knots[i].0, self.knots[j].0);
        let min_x = std::cmp::min(self.knots[i].0, self.knots[j].0);
        let max_y = std::cmp::max(self.knots[i].1, self.knots[j].1);
        let min_y = std::cmp::min(self.knots[i].1, self.knots[j].1);

        std::cmp::max(max_x - min_x, max_y - min_y)
    }

    fn step_follower(&mut self, i: usize) {
        if self.distance(i, i - 1) < 2 {
            return;
        }
        let leader = self.knots[i - 1];
        let follower = self.knots[i];
        match (leader.0.cmp(&follower.0), leader.1.cmp(&follower.1)) {
            (Ordering::Less, Ordering::Less) => {
                self.knots[i].0 -= 1;
                self.knots[i].1 -= 1;
            }
            (Ordering::Less, Ordering::Equal) => {
                self.knots[i].0 -= 1;
            }
            (Ordering::Less, Ordering::Greater) => {
                self.knots[i].0 -= 1;
                self.knots[i].1 += 1;
            }
            (Ordering::Equal, Ordering::Less) => {
                self.knots[i].1 -= 1;
            }
            (Ordering::Equal, Ordering::Equal) => {
                panic!("Invalid case");
            }
            (Ordering::Equal, Ordering::Greater) => {
                self.knots[i].1 += 1;
            }
            (Ordering::Greater, Ordering::Less) => {
                self.knots[i].0 += 1;
                self.knots[i].1 -= 1;
            }
            (Ordering::Greater, Ordering::Equal) => {
                self.knots[i].0 += 1;
            }
            (Ordering::Greater, Ordering::Greater) => {
                self.knots[i].0 += 1;
                self.knots[i].1 += 1;
            }
        }
    }

    fn up(&mut self) {
        self.knots[0].1 += 1;
        for i in 1..self.knots.len() {
            self.step_follower(i);
        }
    }

    fn down(&mut self) {
        self.knots[0].1 -= 1;
        for i in 1..self.knots.len() {
            self.step_follower(i);
        }
    }

    fn right(&mut self) {
        self.knots[0].0 += 1;
        for i in 1..self.knots.len() {
            self.step_follower(i);
        }
    }

    fn left(&mut self) {
        self.knots[0].0 -= 1;
        for i in 1..self.knots.len() {
            self.step_follower(i);
        }
    }

    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Right => self.right(),
            Direction::Left => self.left(),
        }
    }

    fn tail(&self) -> &(usize, usize) {
        self.knots.last().unwrap()
    }
}

pub struct Solver2022_09 {
    steps: Vec<(Direction, usize)>,
}

impl Default for Solver2022_09 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2022_09 {
    fn from(input: &str) -> Self {
        Self {
            steps: input
                .split("\n")
                .map(|step| -> (Direction, usize) {
                    let parts: Vec<&str> = step.split(" ").collect();
                    (
                        match parts[0] {
                            "U" => Direction::Up,
                            "D" => Direction::Down,
                            "L" => Direction::Left,
                            "R" => Direction::Right,
                            _ => unreachable!(),
                        },
                        parts[1].parse().unwrap(),
                    )
                })
                .collect(),
        }
    }
}

impl Solver2022_09 {
    fn solve_part(&self, rope_length: usize) -> usize {
        let mut visit_map = [[false; BOARD_SIZE]; BOARD_SIZE];
        let mut rope = Rope::new(rope_length);
        let tail = rope.tail();
        visit_map[tail.0][tail.1] = true;
        for (direction, count) in &self.steps {
            for _ in 0..*count {
                rope.step(direction);
                let tail = rope.tail();
                visit_map[tail.1][tail.0] = true;
            }
        }
        visit_map.iter().fold(0, |count, row| {
            row.iter().fold(
                count,
                |count, is_visited| if *is_visited { count + 1 } else { count },
            )
        })
    }
}

impl Solver<usize, usize> for Solver2022_09 {
    fn solve_first_part(&self) -> usize {
        self.solve_part(2)
    }

    fn solve_second_part(&self) -> usize {
        self.solve_part(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2022_09::from(
            "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2\
",
        );
        assert_eq!(solver.solve_first_part(), 13);
    }

    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2022_09::from(
            "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20\
",
        );
        assert_eq!(solver.solve_second_part(), 36);
    }
}
