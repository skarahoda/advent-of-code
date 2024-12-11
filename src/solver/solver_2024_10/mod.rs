use std::collections::HashSet;
mod input;
use super::Solver;
use input::INPUT;

pub struct Solver2024_10 {
    map: Vec<Vec<u8>>,
}

impl From<&str> for Solver2024_10 {
    fn from(value: &str) -> Self {
        Self {
            map: value
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect()
                })
                .collect(),
        }
    }
}
impl Default for Solver2024_10 {
    fn default() -> Self {
        INPUT.into()
    }
}

impl Solver2024_10 {
    fn find_trail_destinations(
        &self,
        (x, y): (usize, usize),
        level: u8,
        found: &mut HashSet<(usize, usize)>,
    ) {
        if self
            .map
            .get(y)
            .is_none_or(|row| row.get(x).is_none_or(|&current| current != level))
        {
            return;
        }

        if level == 9 {
            found.insert((x, y));
            return;
        }

        self.find_trail_destinations((x + 1, y), level + 1, found);
        self.find_trail_destinations((x, y + 1), level + 1, found);
        if let Some(next_x) = x.checked_sub(1) {
            self.find_trail_destinations((next_x, y), level + 1, found);
        }
        if let Some(next_y) = y.checked_sub(1) {
            self.find_trail_destinations((x, next_y), level + 1, found);
        }
    }

    fn count_trails(&self, x: usize, y: usize, level: u8) -> usize {
        if self
            .map
            .get(y)
            .is_none_or(|row| row.get(x).is_none_or(|&current| current != level))
        {
            return 0;
        }

        if level == 9 {
            return 1;
        }

        self.count_trails(x + 1, y, level + 1)
            + self.count_trails(x, y + 1, level + 1)
            + self.count_trails(x, y, level + 1)
            + x.checked_sub(1)
                .map_or(0, |next_x| self.count_trails(next_x, y, level + 1))
            + y.checked_sub(1)
                .map_or(0, |next_y| self.count_trails(x, next_y, level + 1))
    }
}

impl Solver<usize, usize> for Solver2024_10 {
    fn solve_first_part(&self) -> usize {
        let mut result = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let mut found = HashSet::new();
                self.find_trail_destinations((x, y), 0, &mut found);
                result += found.len();
            }
        }
        result
    }

    fn solve_second_part(&self) -> usize {
        let mut result = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                result += self.count_trails(x, y, 0);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    #[test]
    fn solve_first_part() {
        assert_eq!(Solver2024_10::from(EXAMPLE).solve_first_part(), 36);
    }

    #[test]
    fn solve_second_part() {
        assert_eq!(Solver2024_10::from(EXAMPLE).solve_second_part(), 81);
    }
}
