use crate::solver::Solver;

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
        Round { player, opponent }
    }

    fn get_result(&self) -> Result {
        match (&self.player, &self.opponent) {
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => Result::Win,
            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock) => Result::Lose,
            (Choice::Rock, Choice::Rock)
            | (Choice::Paper, Choice::Paper)
            | (Choice::Scissors, Choice::Scissors) => Result::Draw,
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

pub struct Solver2022_02 {
    rounds: Vec<(char, char)>,
}

impl Default for Solver2022_02 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2022_02 {
    fn from(input: &str) -> Self {
        let rounds = input
            .lines()
            .map(|line| {
                let choices: Vec<&str> = line.split(" ").collect();
                (
                    choices[1].chars().nth(0).unwrap(),
                    choices[0].chars().nth(0).unwrap(),
                )
            })
            .collect();
        Self { rounds }
    }
}

impl Solver<i32, i32> for Solver2022_02 {
    fn solve_first_part(&self) -> i32 {
        self.rounds
            .iter()
            .map(|(player, opponent)| {
                let player = match player {
                    'X' => Choice::Rock,
                    'Y' => Choice::Paper,
                    'Z' => Choice::Scissors,
                    _ => unreachable!(),
                };
                let opponent = match opponent {
                    'A' => Choice::Rock,
                    'B' => Choice::Paper,
                    'C' => Choice::Scissors,
                    _ => unreachable!(),
                };
                Round::new(player, opponent).get_score()
            })
            .sum()
    }

    fn solve_second_part(&self) -> i32 {
        self.rounds
            .iter()
            .map(|(player, opponent)| {
                let opponent = match opponent {
                    'A' => Choice::Rock,
                    'B' => Choice::Paper,
                    'C' => Choice::Scissors,
                    other => panic!("Illegal argument: {}", other),
                };
                let player = match (opponent, player) {
                    (Choice::Rock, 'Y') | (Choice::Paper, 'X') | (Choice::Scissors, 'Z') => {
                        Choice::Rock
                    }
                    (Choice::Rock, 'Z') | (Choice::Paper, 'Y') | (Choice::Scissors, 'X') => {
                        Choice::Paper
                    }
                    (Choice::Rock, 'X') | (Choice::Paper, 'Z') | (Choice::Scissors, 'Y') => {
                        Choice::Scissors
                    }
                    _ => unreachable!(),
                };
                Round::new(player, opponent).get_score()
            })
            .sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn solve_first_part() {
        let solver = Solver2022_02::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 15);
    }
    #[test]
    fn solve_second_part() {
        let solver = Solver2022_02::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 12);
    }
}
