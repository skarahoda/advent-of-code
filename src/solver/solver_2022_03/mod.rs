use super::Solver;
use std::borrow::Cow;
use std::collections::HashSet;

fn find_common_character(a: &str, b: &str) -> char {
    let mut map: HashSet<char> = HashSet::new();
    for char in a.chars() {
        map.insert(char);
    }
    for char in b.chars() {
        if map.contains(&char) {
            return char;
        }
    }
    panic!("common not found in {} and {}", a, b);
}

fn find_common_character_in_three_strings(a: &str, b: &str, c: &str) -> char {
    let mut map: HashSet<char> = HashSet::new();
    for char in a.chars() {
        map.insert(char);
    }
    let mut second_map: HashSet<char> = HashSet::new();
    for char in b.chars() {
        if map.contains(&char) {
            second_map.insert(char);
        }
    }
    for char in c.chars() {
        if second_map.contains(&char) {
            return char;
        }
    }
    panic!("common not found in {} and {}", a, b);
}

fn character_to_score(c: char) -> u32 {
    let mut buffer = [0; 1];
    c.encode_utf8(&mut buffer);
    match buffer[0] {
        65..=91 => Into::<u32>::into(buffer[0] - 64 + 26),
        97..=122 => Into::<u32>::into(buffer[0] - 96),
        other => panic!("unknown number {}", other),
    }
}

pub struct Solver2022_03<'a> {
    lines: Vec<Cow<'a, str>>,
}

impl<'a> Default for Solver2022_03<'a> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2022_03<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            lines: input.lines().map(|line| Cow::Borrowed(line)).collect(),
        }
    }
}

impl<'a> Solver<u32, u32> for Solver2022_03<'a> {
    fn solve_first_part(&self) -> u32 {
        self.lines
            .iter()
            .map(|line| {
                let (first, second) = line.split_at(line.len() / 2);
                let common = find_common_character(first, second);
                character_to_score(common)
            })
            .sum::<u32>()
    }

    fn solve_second_part(&self) -> u32 {
        let mut result: u32 = 0;

        for chunk in self.lines.chunks(3) {
            let first_line = chunk[0].as_ref();
            let second_line = chunk[1].as_ref();
            let third_line = chunk[2].as_ref();
            let common = find_common_character_in_three_strings(
                first_line.as_ref(),
                second_line.as_ref(),
                third_line.as_ref(),
            );
            result += character_to_score(common);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn solve_first_part() {
        let solver = Solver2022_03::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 157);
    }
    #[test]
    fn solve_second_part() {
        let solver = Solver2022_03::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 70);
    }
}
