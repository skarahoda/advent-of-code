use super::Solver;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::Write;

mod input;
use input::INPUT;

#[derive(Clone)]
pub struct Solver2024_14 {
    robots: Vec<(usize, usize, isize, isize)>,
}

impl Default for Solver2024_14 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

impl From<&str> for Solver2024_14 {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        Self {
            robots: re
                .captures_iter(input)
                .map(|cap| {
                    (
                        cap[1].parse().unwrap(),
                        cap[2].parse().unwrap(),
                        cap[3].parse().unwrap(),
                        cap[4].parse().unwrap(),
                    )
                })
                .collect(),
        }
    }
}

fn move_robot(
    robot: &mut (usize, usize, isize, isize),
    map_width: usize,
    map_height: usize,
    duration: usize,
) {
    let (x, y, vx, vy) = robot;
    let displacement_x = *vx * duration as isize;
    let displacement_y = *vy * duration as isize;
    *x = (displacement_x + *x as isize).rem_euclid(map_width as isize) as usize;
    *y = (displacement_y + *y as isize).rem_euclid(map_height as isize) as usize;
}
impl Solver2024_14 {
    fn write_to_file(&self, path: &str, map_width: usize, map_height: usize) {
        let mut file = std::fs::File::create(path).unwrap();
        let map = self
            .robots
            .iter()
            .map(|&(x, y, ..)| (x, y))
            .collect::<HashSet<(usize, usize)>>();
        for y in 0..map_height {
            for x in 0..map_width {
                if map.contains(&(x, y)) {
                    write!(file, "#").unwrap();
                } else {
                    write!(file, ".").unwrap();
                }
            }
            write!(file, "\n").unwrap();
        }
    }
    fn move_robots(&mut self, map_width: usize, map_height: usize, duration: usize) {
        for robot in &mut self.robots {
            move_robot(robot, map_width, map_height, duration);
        }
    }

    fn calculate_score(&self, map_width: usize, map_height: usize) -> usize {
        let mut quadrants = HashMap::from([((0, 0), 0), ((0, 1), 0), ((1, 0), 0), ((1, 1), 0)]);
        let middle_row = map_height / 2;
        let middle_col = map_width / 2;
        for &(x, y, ..) in &self.robots {
            if x != middle_col && y != middle_row {
                *quadrants
                    .get_mut(&(x / (middle_col + 1), y / (middle_row + 1)))
                    .unwrap() += 1;
            }
        }
        quadrants.values().product()
    }
}

impl Solver<usize, usize> for Solver2024_14 {
    fn solve_first_part(&self) -> usize {
        let mut mutated = self.clone();
        mutated.move_robots(101, 103, 100);
        mutated.calculate_score(101, 103)
    }

    fn solve_second_part(&self) -> usize {
        let mut mutated = self.clone();
        for i in 0..10000 {
            if i % 100 == 0 {
                println!("Iteration: {}", i);
            }
            mutated.write_to_file(&format!("outputs/map_{}.txt", i), 101, 103);
            mutated.move_robots(101, 103, 1);
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3\
";

    #[test]
    fn should_move_a_robot() {
        let mut robot = (2, 4, 2, -3);
        move_robot(&mut robot, 11, 7, 1);
        assert_eq!(robot.0, 4);
        assert_eq!(robot.1, 1);
        let mut robot = (2, 4, 2, -3);
        move_robot(&mut robot, 11, 7, 2);
        assert_eq!(robot.0, 6);
        assert_eq!(robot.1, 5);
        let mut robot = (2, 4, 2, -3);
        move_robot(&mut robot, 11, 7, 3);
        assert_eq!(robot.0, 8);
        assert_eq!(robot.1, 2);
        let mut robot = (2, 4, 2, -3);
        move_robot(&mut robot, 11, 7, 4);
        assert_eq!(robot.0, 10);
        assert_eq!(robot.1, 6);
        let mut robot = (2, 4, 2, -3);
        move_robot(&mut robot, 11, 7, 5);
        assert_eq!(robot.0, 1);
        assert_eq!(robot.1, 3);
    }

    // The map after the hundredth move:
    // ......2..1.
    // ...........
    // 1..........
    // .11........
    // .....1.....
    // ...12......
    // .1....1....
    #[test]
    fn should_move_robots() {
        let mut solver = Solver2024_14::from(EXAMPLE);
        solver.move_robots(11, 7, 100);

        // Check (6,0) has 2 robots
        assert_eq!(
            solver
                .robots
                .iter()
                .filter(|&&(x, y, ..)| x == 6 && y == 0)
                .count(),
            2
        );

        // Check (9,0) has 1 robot
        assert_eq!(
            solver
                .robots
                .iter()
                .filter(|&&(x, y, ..)| x == 9 && y == 0)
                .count(),
            1
        );

        // Check (0,2) has 1 robot
        assert_eq!(
            solver
                .robots
                .iter()
                .filter(|&&(x, y, ..)| x == 0 && y == 2)
                .count(),
            1
        );

        // Check (1,3) and (2,3) each have 1 robot
        assert_eq!(
            solver
                .robots
                .iter()
                .filter(|&&(x, y, ..)| x == 1 && y == 3)
                .count(),
            1
        );
        assert_eq!(
            solver
                .robots
                .iter()
                .filter(|&&(x, y, ..)| x == 2 && y == 3)
                .count(),
            1
        );

        // Check (5,4) has 1 robot
        assert_eq!(
            solver
                .robots
                .iter()
                .filter(|&&(x, y, ..)| x == 5 && y == 4)
                .count(),
            1
        );

        // Check (3,5) and (4,5) have 1 and 2 robots respectively
        assert_eq!(
            solver
                .robots
                .iter()
                .filter(|&&(x, y, ..)| x == 3 && y == 5)
                .count(),
            1
        );
        assert_eq!(
            solver
                .robots
                .iter()
                .filter(|&&(x, y, ..)| x == 4 && y == 5)
                .count(),
            2
        );

        // Check (1,6) and (6,6) each have 1 robot
        assert_eq!(
            solver
                .robots
                .iter()
                .filter(|&&(x, y, ..)| x == 1 && y == 6)
                .count(),
            1
        );
        assert_eq!(
            solver
                .robots
                .iter()
                .filter(|&&(x, y, ..)| x == 6 && y == 6)
                .count(),
            1
        );
    }

    // ..... 2..1.
    // ..... .....
    // 1.... .....
    //
    // ..... .....
    // ...12 .....
    // .1... 1....
    #[test]
    fn should_solve_first_part() {
        let mut solver = Solver2024_14::from(EXAMPLE);
        solver.move_robots(11, 7, 100);
        assert_eq!(solver.calculate_score(11, 7), 12);
    }
}
