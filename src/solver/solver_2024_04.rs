use super::utils;

fn convert_input_to_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
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
    pub fn iter() -> impl Iterator<Item=&'static Self> {
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
}


fn get_char<'a>(input: &'a Vec<Vec<char>>, x: usize, y: usize, distance: usize, direction: &Direction) -> Option<&'a char> {
    let (x, y) = match direction {
        Direction::Left => Some((x.checked_sub(distance)?, y)),
        Direction::Right => Some((x.checked_add(distance)?, y)),
        Direction::Up => Some((x, y.checked_sub(distance)?)),
        Direction::Down => Some((x, y.checked_add(distance)?)),
        Direction::UpLeft => Some((x.checked_sub(distance)?, y.checked_sub(distance)?)),
        Direction::UpRight => Some((x.checked_add(distance)?, y.checked_sub(distance)?)),
        Direction::DownLeft => Some((x.checked_sub(distance)?, y.checked_add(distance)?)),
        Direction::DownRight => Some((x.checked_add(distance)?, y.checked_add(distance)?)),
    }?;
    input.get(y)?.get(x)
}

fn get_chars<'a>(input: &'a Vec<Vec<char>>, x: usize, y: usize, direction: &Direction) -> Option<[&'a char; 4]> {
    let first_char = get_char(input, x, y, 0, direction)?;
    let second_char = get_char(input, x, y, 1, direction)?;
    let third_char = get_char(input, x, y, 2, direction)?;
    let fourth_char = get_char(input, x, y, 3, direction)?;
    Some([first_char, second_char, third_char, fourth_char])
}


fn is_xmas(input: &Vec<Vec<char>>, x: usize, y: usize, direction: &Direction) -> bool {
    let chars = get_chars(input, x, y, direction);
    if chars.is_none() {
        return false;
    }
    let [first_char, second_char, third_char, fourth_char] = chars.unwrap();
    *first_char == 'X' && *second_char == 'M' && *third_char == 'A' && *fourth_char == 'S'
}

struct Cross<'a> {
    center: &'a char,
    top_left: &'a char,
    top_right: &'a char,
    bottom_left: &'a char,
    bottom_right: &'a char,
}

fn get_cross(input: &Vec<Vec<char>>, x: usize, y: usize) -> Option<Cross> {
    Some(Cross {
        center: input.get(y)?.get(x)?,
        top_left: get_char(input, x, y, 1, &Direction::UpLeft)?,
        top_right: get_char(input, x, y, 1, &Direction::UpRight)?,
        bottom_left: get_char(input, x, y, 1, &Direction::DownLeft)?,
        bottom_right: get_char(input, x, y, 1, &Direction::DownRight)?,
    })
}

fn is_x_mas(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let cross = get_cross(input, x, y);
    if cross.is_none() {
        return false;
    }
    let cross = cross.unwrap();
    *cross.center == 'A' && (
        *cross.top_left == 'S' && *cross.top_right == 'S' && *cross.bottom_left == 'M' && *cross.bottom_right == 'M'
            || *cross.top_left == 'M' && *cross.top_right == 'M' && *cross.bottom_left == 'S' && *cross.bottom_right == 'S'
            || *cross.top_left == 'S' && *cross.top_right == 'M' && *cross.bottom_left == 'S' && *cross.bottom_right == 'M'
            || *cross.top_left == 'M' && *cross.top_right == 'S' && *cross.bottom_left == 'M' && *cross.bottom_right == 'S'
    )
}

fn solve_first_part(input: &Vec<Vec<char>>) -> i32 {
    let height = input.len();
    let width = input[0].len();

    let mut result = 0;
    for y in 0..height {
        for x in 0..width {
            for direction in Direction::iter() {
                if is_xmas(input, x, y, direction) {
                    result += 1;
                }
            }
        }
    }

    result
}
fn solve_second_part(input: &Vec<Vec<char>>) -> i32 {
    let height = input.len();
    let width = input[0].len();

    let mut result = 0;
    for y in 0..height {
        for x in 0..width {
            if is_x_mas(input, x, y) {
                result += 1;
            }
        }
    }

    result
}

pub fn solve() -> (i32, i32) {
    let input = utils::get_input("inputs/2024_04.txt");
    let input = convert_input_to_matrix(&input);
    (
        solve_first_part(&input),
        solve_second_part(&input),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_first_part() {
        assert_eq!(
            super::solve_first_part(&super::convert_input_to_matrix(
                r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            )),
            18
        );
    }

    #[test]
    fn solve_second_part() {
        assert_eq!(
            super::solve_second_part(&super::convert_input_to_matrix(
                r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            )),
            9
        );
    }
}
