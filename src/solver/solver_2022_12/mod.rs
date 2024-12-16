use super::Solver;
use std::collections::VecDeque;

pub struct Solver2022_12 {
    points: Vec<Vec<usize>>,
    start_coordinates: (usize, usize),
    end_coordinates: (usize, usize),
}

impl Default for Solver2022_12 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2022_12 {
    fn from(input: &str) -> Self {
        let rows: Vec<&str> = input.split("\n").collect();
        let x = rows[0].len();
        let y = rows.len();
        let mut start_coordinates = (0, 0);
        let mut end_coordinates = (0, 0);
        let mut points = vec![vec![0; x]; y];
        for (i, row) in rows.iter().enumerate() {
            for (j, char) in row.chars().enumerate() {
                match char {
                    'S' => {
                        start_coordinates = (j, i);
                        points[i][j] = 0;
                    }
                    'E' => {
                        end_coordinates = (j, i);
                        points[i][j] = 25;
                    }
                    'a'..='z' => {
                        points[i][j] = (char as usize) - ('a' as usize);
                    }
                    _ => unreachable!(),
                }
            }
        }
        Self {
            start_coordinates,
            end_coordinates,
            points,
        }
    }
}

impl Solver2022_12 {
    fn get_available_neighbours(
        &self,
        (x, y): (usize, usize),
        distance_map: &Vec<Vec<usize>>,
    ) -> Vec<(usize, usize)> {
        let available_height = self.points[y][x] + 1;
        let mut result = Vec::new();
        if y + 1 < self.points.len()
            && self.points[y + 1][x] <= available_height
            && distance_map[y + 1][x] == usize::MAX
        {
            result.push((x, y + 1));
        }
        if y > 0
            && self.points[y - 1][x] <= available_height
            && distance_map[y - 1][x] == usize::MAX
        {
            result.push((x, y - 1));
        }
        if x + 1 < self.points[y].len()
            && self.points[y][x + 1] <= available_height
            && distance_map[y][x + 1] == usize::MAX
        {
            result.push((x + 1, y));
        }
        if x > 0
            && self.points[y][x - 1] <= available_height
            && distance_map[y][x - 1] == usize::MAX
        {
            result.push((x - 1, y));
        }
        result
    }

    fn get_available_neighbours_reverse(
        &self,
        (x, y): (usize, usize),
        distance_map: &Vec<Vec<usize>>,
    ) -> Vec<(usize, usize)> {
        let available_height = self.points[y][x] - 1;
        let mut result = Vec::new();
        if y + 1 < self.points.len()
            && self.points[y + 1][x] >= available_height
            && distance_map[y + 1][x] == usize::MAX
        {
            result.push((x, y + 1));
        }
        if y > 0
            && self.points[y - 1][x] >= available_height
            && distance_map[y - 1][x] == usize::MAX
        {
            result.push((x, y - 1));
        }
        if x + 1 < self.points[y].len()
            && self.points[y][x + 1] >= available_height
            && distance_map[y][x + 1] == usize::MAX
        {
            result.push((x + 1, y));
        }
        if x > 0
            && self.points[y][x - 1] >= available_height
            && distance_map[y][x - 1] == usize::MAX
        {
            result.push((x - 1, y));
        }
        result
    }

    fn find_shortest_path_from_start(&self) -> usize {
        let mut distance_map = vec![vec![usize::MAX; self.points[0].len()]; self.points.len()];
        distance_map[self.start_coordinates.1][self.start_coordinates.0] = 0;
        let mut queue = VecDeque::from([self.start_coordinates]);

        while let Some(current_position) = queue.pop_front() {
            let current_distance = distance_map[current_position.1][current_position.0];
            let neighbours = self.get_available_neighbours(current_position, &distance_map);
            for neighbour in neighbours {
                distance_map[neighbour.1][neighbour.0] = current_distance + 1;
                queue.push_back(neighbour);
            }
        }
        distance_map[self.end_coordinates.1][self.end_coordinates.0]
    }

    fn find_shortest_path_from_anywhere(&self) -> usize {
        let mut distance_map = vec![vec![usize::MAX; self.points[0].len()]; self.points.len()];
        distance_map[self.end_coordinates.1][self.end_coordinates.0] = 0;
        let mut queue = VecDeque::from([self.end_coordinates]);

        while let Some(current_position) = queue.pop_front() {
            let current_distance = distance_map[current_position.1][current_position.0];
            let neighbours = self.get_available_neighbours_reverse(current_position, &distance_map);
            for neighbour in neighbours {
                if self.points[neighbour.1][neighbour.0] == 0 {
                    return current_distance + 1;
                }
                distance_map[neighbour.1][neighbour.0] = current_distance + 1;
                queue.push_back(neighbour);
            }
        }
        panic!("Couldn't find a solution")
    }
}

impl Solver<usize, usize> for Solver2022_12 {
    fn solve_first_part(&self) -> usize {
        self.find_shortest_path_from_start()
    }

    fn solve_second_part(&self) -> usize {
        self.find_shortest_path_from_anywhere()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
        Sabqponm\n\
        abcryxxl\n\
        accszExk\n\
        acctuvwj\n\
        abdefghi";

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2022_12::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 31);
    }

    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2022_12::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 29);
    }
}
