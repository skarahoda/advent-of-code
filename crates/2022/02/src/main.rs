use std::fs;

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

fn main() {
    let mut content = fs::read_to_string("inputs/2022_02.txt")
        .expect("Should have been able to read the file");
    if content.chars().last().unwrap() == "\n".chars().next().unwrap() {
        content.pop();
    }
    let result: i32 = content.split("\n").map(|round| {
        let choices: Vec<&str> = round.split(" ").collect();
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
    }).sum();
    dbg!(result);
}
