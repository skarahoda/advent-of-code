fn solve_first_part(input: &str) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            other => panic!("Illegal argument: {}", other),
        })
        .sum()
}
fn solve_second_part(input: &str) -> usize {
    let mut score = 0;

    for (i, char) in input.chars().enumerate() {
        match char {
            '(' => score += 1,
            ')' => score -= 1,
            other => panic!("Illegal argument: {}", other),
        }
        if score < 0 {
            return i + 1;
        }
    }
    panic!("Solution not found");
}

pub fn solve() -> (i32, usize) {
    (
        solve_first_part(include_str!("input.txt")),
        solve_second_part(include_str!("input.txt")),
    )
}

#[cfg(test)]
mod first_part {
    use super::*;
    #[test]
    fn same_number_of_closing_and_opening_parenthesis() {
        assert_eq!(solve_first_part("()()"), 0);
    }
    #[test]
    fn has_more_closing_parenthesis() {
        assert_eq!(solve_first_part("()()))"), -2);
    }
    #[test]
    fn has_more_opening_parenthesis() {
        assert_eq!(solve_first_part("()()(("), 2);
    }
}

#[cfg(test)]
mod second_part {
    use super::*;
    #[test]
    fn enter_basement_with_first_character() {
        assert_eq!(solve_second_part(")"), 1);
    }
    #[test]
    fn enter_basement_with_last_character() {
        assert_eq!(solve_second_part("(()))"), 5);
    }
    #[test]
    fn enter_basement_in_the_middle() {
        assert_eq!(solve_second_part("()))()())()())"), 3);
    }
}
