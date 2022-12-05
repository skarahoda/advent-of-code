use std::fs;

pub fn get_input() -> String {
    let mut content = fs::read_to_string("inputs/2015_01.txt")
        .expect("Should have been able to read the file");
    if content.chars().last().unwrap() == "\n".chars().next().unwrap() {
        content.pop();
    }
    content
}

pub fn solve_first_part() -> i32 {
    let input = get_input();
    input.chars().fold(
        0,
        |score, c| {
            match c {
                '(' => score+1,
                ')' => score-1,
                other => panic!("Illegal argument: {}", other)
            }
        }
    )
}
pub fn solve_second_part() -> usize {
    let input = get_input();
    let mut score = 0;
    for (i, char) in input.chars().enumerate() {
        match char {
            '(' => score+=1,
            ')' => score-=1,
            other => panic!("Illegal argument: {}", other)
        }
        if score < 0 {
            return i + 1;
        }
    }
    panic!("Solution not found");
}
