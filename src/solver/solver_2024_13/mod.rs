use super::Solver;
use regex::Regex;

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

fn calc_det(a: (usize, usize), b: (usize, usize)) -> usize {
    (a.0 * b.1).abs_diff(a.1 * b.0)
}

impl ClawMachine {
    fn get_with_min_cost(&self) -> Option<(usize, usize)> {
        let det = calc_det(self.button_a, self.button_b);
        if det == 0 {
            let button_a_steps = div_without_remainder(self.prize.0, self.button_a.0)?;
            let button_b_steps = div_without_remainder(self.prize.1, self.button_a.1)?;
            if button_a_steps * 3 < button_b_steps {
                return Some((button_a_steps, 0));
            } else {
                return Some((0, button_b_steps));
            }
        }
        let det_a = calc_det(self.button_a, self.prize);
        let det_b = calc_det(self.button_b, self.prize);

        Some((
            div_without_remainder(det_b, det)?,
            div_without_remainder(det_a, det)?,
        ))
    }
}

pub struct Solver2024_13 {
    machines: Vec<ClawMachine>,
}

impl Default for Solver2024_13 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
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
                // println!("{:?} {:?}", machine, machine.get_with_min_cost());
                machine.get_with_min_cost()
            })
            .map(|(a, b)| calculate_token_cost(&(a, b)))
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
