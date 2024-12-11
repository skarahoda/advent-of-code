use super::Solver;
use std::collections::HashMap;
mod input;
use input::INPUT;

pub struct Solver202401 {
    left_list: Vec<i32>,
    right_list: Vec<i32>,
}

impl Default for Solver202401 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

impl From<&str> for Solver202401 {
    fn from(input: &str) -> Self {
        let mut left_list = Vec::new();
        let mut right_list = Vec::new();

        for line in input.lines() {
            let mut line = line.split_whitespace();
            let left = line.next().unwrap().parse().unwrap();
            let right = line.next().unwrap().parse().unwrap();
            left_list.push(left);
            right_list.push(right);
        }

        Self {
            left_list,
            right_list,
        }
    }
}

impl Solver<i32, i32> for Solver202401 {
    fn solve_first_part(&self) -> i32 {
        let mut left_list = self.left_list.clone();
        left_list.sort();
        let mut right_list = self.right_list.clone();
        right_list.sort();

        left_list
            .iter()
            .enumerate()
            .map(|(index, left)| (left - right_list[index]).abs())
            .sum()
    }

    fn solve_second_part(&self) -> i32 {
        let count_map = self
            .right_list
            .iter()
            .fold(HashMap::new(), |mut map, right| {
                map.entry(right)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                map
            });

        self.left_list
            .iter()
            .map(|left| count_map.get(left).unwrap_or(&0) * left)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE: &str = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";

    #[test]
    fn solve_first_part() {
        let solver = Solver202401::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 11);
    }

    #[test]
    fn solve_second_part() {
        let solver = Solver202401::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 31);
    }
}
