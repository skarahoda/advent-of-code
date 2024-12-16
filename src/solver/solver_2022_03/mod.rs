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

fn solve_first_part(input: &str) -> u32 {
    input
        .split("\n")
        .map(|row: &str| {
            let (first, second) = row.split_at(row.len() / 2);
            let common = find_common_character(first, second);
            character_to_score(common)
        })
        .sum::<u32>()
}

fn solve_second_part(input: &str) -> u32 {
    let rows: Vec<&str> = input.split("\n").collect();

    let mut result: u32 = 0;

    for i in 0..rows.len() / 3 {
        let first_row = rows[3 * i];
        let second_row = rows[3 * i + 1];
        let third_row = rows[3 * i + 2];
        let common = find_common_character_in_three_strings(first_row, second_row, third_row);
        result += character_to_score(common);
    }

    result
}

pub fn solve() -> (u32, u32) {
    (
        solve_first_part(include_str!("input.txt")),
        solve_second_part(include_str!("input.txt")),
    )
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
    #[test]
    fn solve_first_part() {
        assert_eq!(super::solve_first_part(EXAMPLE), 157);
    }
    #[test]
    fn solve_second_part() {
        assert_eq!(super::solve_second_part(EXAMPLE), 70);
    }
}
