use std::collections::HashSet;
use std::fs;

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
    let mut buffer =  [0; 1];
    c.encode_utf8(&mut buffer);
    match buffer[0] {
        65..=91 => Into::<u32>::into(buffer[0] - 64 + 26),
        97..=122 => Into::<u32>::into(buffer[0] - 96),
        other => panic!("unknown number {}", other)
    }
}

pub fn solve_first_part() -> u32 {
    let mut content = fs::read_to_string("inputs/2022_03.txt")
        .expect("Should have been able to read the file");
    if content.chars().last().unwrap() == "\n".chars().next().unwrap() {
        content.pop();
    }

    content.split("\n").map(
        |row: &str| {
            let (first, second) = row.split_at(row.len()/2);
            let common = find_common_character(first, second);
            character_to_score(common)
        }
    ).sum::<u32>()
}

pub fn solve_second_part() -> u32 {
    let mut content = fs::read_to_string("inputs/2022_03.txt")
        .expect("Should have been able to read the file");
    if content.chars().last().unwrap() == "\n".chars().next().unwrap() {
        content.pop();
    }
    let rows:Vec<&str> = content.split("\n").collect();


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
