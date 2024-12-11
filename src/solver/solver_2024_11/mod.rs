use super::Solver;
use std::collections::LinkedList;

mod input;
use input::INPUT;

#[derive(Clone)]
pub struct Solver2024_11 {
    rocks: LinkedList<usize>,
}

impl<'a> Default for Solver2024_11 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

impl<'a> From<&'a str> for Solver2024_11 {
    fn from(input: &'a str) -> Self {
        Self {
            rocks: input
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }
}

impl Solver2024_11 {
    fn blink(&mut self) {
        let mut cursor = self.rocks.cursor_front_mut();
        while let Some(node) = cursor.current() {
            let node_str = node.to_string();
            if node == &0 {
                *node = 1;
            } else if node_str.len() % 2 == 0 {
                let left = node_str
                    .chars()
                    .take(node_str.len() / 2)
                    .collect::<String>()
                    .parse()
                    .unwrap();
                let right = node_str
                    .chars()
                    .skip(node_str.len() / 2)
                    .collect::<String>()
                    .parse()
                    .unwrap();
                *node = right;
                cursor.insert_before(left);
            } else {
                *node *= 2024;
            }
            cursor.move_next();
        }
    }
}

impl Solver<usize, usize> for Solver2024_11 {
    fn solve_first_part(&self) -> usize {
        let mut muted = self.clone();
        for _ in 0..25 {
            muted.blink();
        }
        muted.rocks.len()
    }

    fn solve_second_part(&self) -> usize {
        panic!("second part is not working");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "125 17";

    #[test]
    fn should_solve_first_part() {
        let solver = Solver2024_11::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 55312);
    }
}
