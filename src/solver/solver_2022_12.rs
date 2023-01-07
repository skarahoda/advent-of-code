use super::utils;
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
struct Maze {
    pub points: Vec<Vec<usize>>,
    pub start_coordinates: (usize, usize),
    pub end_coordinates: (usize, usize),
}

impl Maze {
    fn new(x: usize, y: usize) -> Self {
        Self {
            points: vec![vec![0; x]; y],
            start_coordinates: (0, 0),
            end_coordinates: (0, 0),
        }
    }

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

fn get_maze(input: &str) -> Maze {
    let rows: Vec<&str> = input.split("\n").collect();
    let x = rows[0].len();
    let y = rows.len();
    let mut result = Maze::new(x, y);
    for (i, row) in rows.iter().enumerate() {
        for (j, char) in row.chars().enumerate() {
            match char {
                'S' => {
                    result.start_coordinates = (j, i);
                    result.points[i][j] = 0;
                }
                'E' => {
                    result.end_coordinates = (j, i);
                    result.points[i][j] = 25;
                }
                'a'..='z' => {
                    result.points[i][j] = (char as usize) - ('a' as usize);
                }
                other => panic!("Illegal argument: {}", other),
            }
        }
    }
    result
}

fn solve_first_part(maze: &Maze) -> usize {
    maze.find_shortest_path_from_start()
}

fn solve_second_part(maze: &Maze) -> usize {
    maze.find_shortest_path_from_anywhere()
}

pub fn solve() -> (usize, usize) {
    let maze = get_maze(&utils::get_input("inputs/2022_12.txt"));
    (solve_first_part(&maze), solve_second_part(&maze))
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
    fn should_parse_input() {
        assert_eq!(
            get_maze(EXAMPLE),
            Maze {
                points: vec![
                    vec![0, 0, 1, 16, 15, 14, 13, 12],
                    vec![0, 1, 2, 17, 24, 23, 23, 11],
                    vec![0, 2, 2, 18, 25, 25, 23, 10],
                    vec![0, 2, 2, 19, 20, 21, 22, 9],
                    vec![0, 1, 3, 4, 5, 6, 7, 8]
                ],
                start_coordinates: (0, 0),
                end_coordinates: (5, 2)
            }
        )
    }

    #[test]
    fn should_solve_first_part_example() {
        let maze = get_maze(EXAMPLE);
        assert_eq!(solve_first_part(&maze), 31);
    }
    #[test]
    fn should_solve_second_part_example() {
        let maze = get_maze(EXAMPLE);
        assert_eq!(solve_second_part(&maze), 29);
    }
}
