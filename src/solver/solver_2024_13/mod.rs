use super::Solver;
use regex::Regex;
mod input;
use input::INPUT;

#[derive(Debug)]
struct ClawMachine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

fn calculate_token_cost((a, b): &(usize, usize)) -> usize {
    a * 3 + b
}

fn div_without_remainder(a: usize, b: usize) -> Option<usize> {
    if a % b == 0 {
        Some(a / b)
    } else {
        None
    }
}
impl ClawMachine {
    fn get_result_with_all_button_b(&self) -> Option<usize> {
        let steps_x = div_without_remainder(self.prize.0, self.button_b.0)?;
        let steps_y = div_without_remainder(self.prize.1, self.button_b.1)?;

        if steps_x == steps_y {
            Some(steps_x)
        } else {
            None
        }
    }

    fn get_with_min_cost(&self) -> Option<(usize, usize)> {
        if self.prize.0 == 0 && self.prize.1 == 0 {
            return Some((0, 0));
        }

        let recursion = || {
            let mutated_prize = Self {
                prize: (
                    self.prize.0.checked_sub(self.button_a.0)?,
                    self.prize.1.checked_sub(self.button_a.1)?,
                ),
                ..*self
            };
            mutated_prize.get_with_min_cost().map(|(a, b)| (a + 1, b))
        };

        vec![
            self.get_result_with_all_button_b().map(|b| (0, b)),
            recursion(),
        ]
        .iter()
        .flatten()
        .min_by(|&lhs, &rhs| calculate_token_cost(lhs).cmp(&calculate_token_cost(rhs)))
        .cloned()
    }
}

pub struct Solver2024_13 {
    machines: Vec<ClawMachine>,
}

impl Default for Solver2024_13 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

impl From<&str> for Solver2024_13 {
    fn from(input: &str) -> Self {
        let re = Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .unwrap();

        Self {
            machines: re
                .captures_iter(input)
                .map(|cap| ClawMachine {
                    button_a: (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                    button_b: (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
                    prize: (cap[5].parse().unwrap(), cap[6].parse().unwrap()),
                })
                .collect(),
        }
    }
}

impl Solver2024_13 {
    fn calculate_token_cost(&self) -> usize {
        self.machines
            .iter()
            .filter_map(|machine| {
                machine
                    .get_with_min_cost()
                    .map(|val| calculate_token_cost(&val))
            })
            .sum()
    }
}

impl Solver<usize, usize> for Solver2024_13 {
    fn solve_first_part(&self) -> usize {
        self.calculate_token_cost()
    }

    fn solve_second_part(&self) -> usize {
        let mutated_solver = Self {
            machines: self
                .machines
                .iter()
                .map(|machine| ClawMachine {
                    prize: (
                        machine.prize.0 + 10000000000000,
                        machine.prize.1 + 10000000000000,
                    ),
                    ..*machine
                })
                .collect(),
        };
        mutated_solver.calculate_token_cost()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279\
";

    #[test]
    fn should_solve_first_part() {
        let solver = Solver2024_13::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 480);
    }
}
