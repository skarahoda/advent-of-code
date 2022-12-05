use super::utils;

enum Result {
    Win,
    Draw,
    Lose,
}

#[derive(Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    player: Choice,
    opponent: Choice,
}

impl Round {
    fn new(player: Choice, opponent: Choice) -> Self {
        Round {
            player,
            opponent
        }
    }

    fn get_result(&self) -> Result {
        match (&self.player, &self.opponent) {
            (Choice::Rock, Choice::Scissors) |
            (Choice::Paper, Choice::Rock) |
            (Choice::Scissors, Choice::Paper) => Result::Win,
            (Choice::Rock, Choice::Paper) |
            (Choice::Paper, Choice::Scissors) |
            (Choice::Scissors, Choice::Rock)  => Result::Lose,
            (Choice::Rock, Choice::Rock) |
            (Choice::Paper, Choice::Paper) |
            (Choice::Scissors, Choice::Scissors) => Result::Draw,
        }
    }

    fn get_score(&self) -> i32 {
        let score_from_result = match self.get_result() {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lose => 0,
        };
        let score_from_choice = match self.player {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };
        score_from_result + score_from_choice
    }
}

fn solve_first_part(input: &str) -> i32 {
    input.split("\n").map(|row| {
        let choices: Vec<&str> = row.split(" ").collect();
        let opponent = match choices[0] {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            other => panic!("Illegal argument: {}", other)
        };
        let player = match choices[1] {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            other => panic!("Illegal argument: {}", other)
        };
        Round::new(player, opponent).get_score()
    }).sum()
}

fn solve_second_part(input: &str) -> i32 {
    input.split("\n").map(|row| {
        let choices: Vec<&str> = row.split(" ").collect();
        let opponent = match choices[0] {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            other => panic!("Illegal argument: {}", other)
        };
        let player = match (opponent, choices[1]) {
            (Choice::Rock, "Y") |
            (Choice::Paper, "X") |
            (Choice::Scissors, "Z") => Choice::Rock,
            (Choice::Rock, "Z") |
            (Choice::Paper, "Y") |
            (Choice::Scissors, "X") => Choice::Paper,
            (Choice::Rock, "X") |
            (Choice::Paper, "Z") |
            (Choice::Scissors, "Y") => Choice::Scissors,
            (_, other) => panic!("Illegal argument: {}", other)
        };
        Round::new(player, opponent).get_score()
    }).sum()
}

pub fn solve() -> (i32, i32) {
    let input = utils::get_input("inputs/2022_02.txt");
    (
        solve_first_part(&input[..]),
        solve_second_part(&input[..]),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_first_part() {
        assert_eq!(super::solve_first_part("A Y\nB X\nC Z"), 15);
    }
    #[test]
    fn solve_second_part() {
        assert_eq!(super::solve_second_part("A Y\nB X\nC Z"), 12);
    }
}
