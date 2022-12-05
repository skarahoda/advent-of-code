use super::utils;

fn solve_first_part(input: &str) -> i32 {
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
fn solve_second_part(input: &str) -> usize {
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

pub fn solve() -> (i32, usize) {
    let input = utils::get_input("inputs/2015_01.txt");
    (
        solve_first_part(&input[..]),
        solve_second_part(&input[..])
    )
}
