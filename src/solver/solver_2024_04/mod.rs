use super::Solver;
mod input;
use input::INPUT;

pub struct Solver2024_04 {
    matrix: Vec<Vec<char>>,
}

impl From<&str> for Solver2024_04 {
    fn from(input: &str) -> Self {
        Self {
            matrix: input
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect(),
        }
    }
}

impl Default for Solver2024_04 {
    fn default() -> Self {
        INPUT.into()
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = &'static Self> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ];
        DIRECTIONS.iter()
    }

    pub fn get_next_position(&self, x: usize, y: usize, distance: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Left => Some((x.checked_sub(distance)?, y)),
            Direction::Right => Some((x.checked_add(distance)?, y)),
            Direction::Up => Some((x, y.checked_sub(distance)?)),
            Direction::Down => Some((x, y.checked_add(distance)?)),
            Direction::UpLeft => Some((x.checked_sub(distance)?, y.checked_sub(distance)?)),
            Direction::UpRight => Some((x.checked_add(distance)?, y.checked_sub(distance)?)),
            Direction::DownLeft => Some((x.checked_sub(distance)?, y.checked_add(distance)?)),
            Direction::DownRight => Some((x.checked_add(distance)?, y.checked_add(distance)?)),
        }
    }
}

impl Solver2024_04 {
    fn get_char(
        &self,
        x: usize,
        y: usize,
        distance: usize,
        direction: &Direction,
    ) -> Option<&char> {
        let (x, y) = direction.get_next_position(x, y, distance)?;
        self.matrix.get(y)?.get(x)
    }

    fn get_chars(&self, x: usize, y: usize, direction: &Direction) -> Option<[&char; 4]> {
        let first_char = self.get_char(x, y, 0, direction)?;
        let second_char = self.get_char(x, y, 1, direction)?;
        let third_char = self.get_char(x, y, 2, direction)?;
        let fourth_char = self.get_char(x, y, 3, direction)?;
        Some([first_char, second_char, third_char, fourth_char])
    }

    fn is_xmas(&self, x: usize, y: usize, direction: &Direction) -> bool {
        let chars = self.get_chars(x, y, direction);
        if let Some([first_char, second_char, third_char, fourth_char]) = chars {
            *first_char == 'X' && *second_char == 'M' && *third_char == 'A' && *fourth_char == 'S'
        } else {
            false
        }
    }

    fn get_cross(&self, x: usize, y: usize) -> Option<Cross> {
        Some(Cross {
            center: self.matrix.get(y)?.get(x)?,
            top_left: self.get_char(x, y, 1, &Direction::UpLeft)?,
            top_right: self.get_char(x, y, 1, &Direction::UpRight)?,
            bottom_left: self.get_char(x, y, 1, &Direction::DownLeft)?,
            bottom_right: self.get_char(x, y, 1, &Direction::DownRight)?,
        })
    }
}

impl Solver<i32, i32> for Solver2024_04 {
    fn solve_first_part(&self) -> i32 {
        let height = self.matrix.len();
        let width = self.matrix[0].len();

        let mut result = 0;
        for y in 0..height {
            for x in 0..width {
                for direction in Direction::iter() {
                    if self.is_xmas(x, y, direction) {
                        result += 1;
                    }
                }
            }
        }
        result
    }

    fn solve_second_part(&self) -> i32 {
        let height = self.matrix.len();
        let width = self.matrix[0].len();

        let mut result = 0;
        for y in 0..height {
            for x in 0..width {
                if self.get_cross(x, y).is_some_and(|cross| cross.is_x_mas()) {
                    result += 1;
                }
            }
        }
        result
    }
}

struct Cross<'a> {
    center: &'a char,
    top_left: &'a char,
    top_right: &'a char,
    bottom_left: &'a char,
    bottom_right: &'a char,
}

impl Cross<'_> {
    fn is_x_mas(&self) -> bool {
        *self.center == 'A'
            && ((*self.top_left == 'S'
                && *self.top_right == 'S'
                && *self.bottom_left == 'M'
                && *self.bottom_right == 'M')
                || (*self.top_left == 'M'
                    && *self.top_right == 'M'
                    && *self.bottom_left == 'S'
                    && *self.bottom_right == 'S')
                || (*self.top_left == 'S'
                    && *self.top_right == 'M'
                    && *self.bottom_left == 'S'
                    && *self.bottom_right == 'M')
                || (*self.top_left == 'M'
                    && *self.top_right == 'S'
                    && *self.bottom_left == 'M'
                    && *self.bottom_right == 'S'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn solve_first_part() {
        assert_eq!(Solver2024_04::from(EXAMPLE).solve_first_part(), 18);
    }

    #[test]
    fn solve_second_part() {
        assert_eq!(Solver2024_04::from(EXAMPLE).solve_second_part(), 9);
    }
}
