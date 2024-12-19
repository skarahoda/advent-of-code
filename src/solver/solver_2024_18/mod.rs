use crate::solver::Solver;
use std::collections::{HashSet, VecDeque};

type Coordinate = (usize, usize);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> Vec<Self> {
        vec![Self::Up, Self::Down, Self::Left, Self::Right]
    }
}

impl From<Direction> for (isize, isize) {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Clone)]
pub struct Solver2024_18 {
    walls: Vec<Coordinate>,
    number_of_walls: usize,
    width: usize,
    height: usize,
}

impl Default for Solver2024_18 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2024_18 {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split("\n\n").collect();
        let first_part: Vec<usize> = parts[0]
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect();
        Self {
            width: first_part[0],
            height: first_part[1],
            number_of_walls: first_part[2],
            walls: parts[1]
                .lines()
                .map(|line| {
                    let nums: Vec<usize> =
                        line.split(",").map(|num| num.parse().unwrap()).collect();
                    (nums[0], nums[1])
                })
                .collect(),
        }
    }
}

impl Solver2024_18 {
    fn get_cell(
        &self,
        walls: &HashSet<Coordinate>,
        location: Coordinate,
        direction: Direction,
    ) -> Option<Coordinate> {
        let (x, y) = location;
        let (dx, dy) = direction.into();
        let (x, y) = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);
        if x < self.width && y < self.height && !walls.contains(&(x, y)) {
            Some((x, y))
        } else {
            None
        }
    }

    fn find_shortest_path(&self, number_of_walls: usize) -> Option<usize> {
        let end = (self.width - 1, self.height - 1);
        let walls: HashSet<Coordinate> = self.walls.iter().take(number_of_walls).cloned().collect();
        let mut visited: HashSet<Coordinate> = HashSet::new();
        let mut queue: VecDeque<(Coordinate, usize)> = vec![((0, 0), 0)].into();
        while let Some((current, distance)) = queue.pop_front() {
            for direction in Direction::all() {
                if let Some(next) = self.get_cell(&walls, current, direction) {
                    if visited.contains(&next) {
                        continue;
                    }
                    if next == end {
                        return Some(distance + 1);
                    } else {
                        queue.push_back((next, distance + 1));
                        visited.insert(next);
                    }
                }
            }
        }
        None
    }

    fn binary_search_for_blocker(&self) -> Option<usize> {
        let mut left = 0;
        let mut right = self.walls.len();
        while left < right {
            let mid = (left + right) / 2;
            if self.find_shortest_path(mid).is_some() {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        Some(left)
    }
}

impl Solver<usize, String> for Solver2024_18 {
    fn solve_first_part(&self) -> usize {
        self.find_shortest_path(self.number_of_walls).unwrap()
    }

    fn solve_second_part(&self) -> String {
        let index = self.binary_search_for_blocker().unwrap();
        let (x, y) = self.walls[index - 1];
        format!("{},{}", x, y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "\
7,7,12

5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0\
";
    #[test]
    fn test_solve_first_part() {
        let solver = Solver2024_18::from(EXAMPLE);

        assert_eq!(solver.solve_first_part(), 22);
    }

    #[test]
    fn test_solve_second_part() {
        let solver = Solver2024_18::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), "6,1");
    }
}
