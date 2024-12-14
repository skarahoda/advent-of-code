use super::Solver;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::Write;

mod input;
use input::INPUT;

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

#[derive(Clone)]
pub struct Solver2024_14 {
    robots: Vec<(usize, usize, isize, isize)>,
    width: usize,
    height: usize,
}

impl Default for Solver2024_14 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

impl From<&str> for Solver2024_14 {
    fn from(input: &str) -> Self {
        let re_size = Regex::new(r"size=(\d+),(\d+)").unwrap();
        let re_robot = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let size_captures = re_size.captures(input).unwrap();
        Self {
            robots: re_robot
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
            width: size_captures[1].parse().unwrap(),
            height: size_captures[2].parse().unwrap(),
        }
    }
}

impl Solver2024_14 {
    fn write_to_file(&self, path: &str) {
        let mut file = std::fs::File::create(path).unwrap();
        let map = self
            .robots
            .iter()
            .map(|&(x, y, ..)| (x, y))
            .collect::<HashSet<(usize, usize)>>();
        for y in 0..self.height {
            for x in 0..self.width {
                if map.contains(&(x, y)) {
                    write!(file, "#").unwrap();
                } else {
                    write!(file, ".").unwrap();
                }
            }
            write!(file, "\n").unwrap();
        }
    }

    fn move_robots(&mut self, duration: usize) {
        for robot in &mut self.robots {
            move_robot(robot, self.width, self.height, duration);
        }
    }

    fn calculate_score(&self) -> usize {
        let mut quadrants = HashMap::from([((0, 0), 0), ((0, 1), 0), ((1, 0), 0), ((1, 1), 0)]);
        let middle_row = self.height / 2;
        let middle_col = self.width / 2;
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
        mutated.move_robots(100);
        mutated.calculate_score()
    }

    fn solve_second_part(&self) -> usize {
        let mut mutated = self.clone();
        for i in 0..10000 {
            if i % 100 == 0 {
                println!("Iteration: {}", i);
            }
            mutated.write_to_file(&format!("outputs/map_{}.txt", i));
            mutated.move_robots(1);
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
size=11,7
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
    fn should_solve_first_part() {
        let solver = Solver2024_14::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 12);
    }
}
