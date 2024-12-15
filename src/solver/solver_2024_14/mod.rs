use super::Solver;
use regex::Regex;
use std::collections::HashMap;

mod input;
use input::INPUT;

mod christmas_tree;
use christmas_tree::CHRISTMAS_TREE;

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
    fn has_christmas_tree(&self) -> bool {
        let mut map = vec![vec![false; self.width]; self.height];
        for &(x, y, ..) in &self.robots {
            if x < self.width && y < self.height {
                map[y][x] = true;
            }
        }

        for x in 0..self.width - CHRISTMAS_TREE[0].len() + 1 {
            for y in 0..self.height - CHRISTMAS_TREE.len() + 1 {
                let window = map
                    .iter()
                    .skip(y)
                    .take(CHRISTMAS_TREE.len())
                    .map(|row| row.iter().skip(x).take(CHRISTMAS_TREE[0].len()));
                let mut is_christmas_tree = true;
                for (y, row) in window.enumerate() {
                    if !row.eq(&CHRISTMAS_TREE[y]) {
                        is_christmas_tree = false;
                        break;
                    }
                }
                if is_christmas_tree {
                    return true;
                }
            }
        }
        false
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
        let mut i = 0;
        loop {
            if mutated.has_christmas_tree() {
                return i;
            }
            mutated.move_robots(1);
            i += 1;
        }
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
