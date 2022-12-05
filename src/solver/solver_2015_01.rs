use super::utils;

pub fn solve_first_part() -> i32 {
    let input = utils::get_input("inputs/2015_01.txt");

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
    let input = utils::get_input("inputs/2015_01.txt");
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
