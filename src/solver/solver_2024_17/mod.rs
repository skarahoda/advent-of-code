use crate::solver::Solver;
use regex::Regex;

#[derive(Clone)]
pub struct Solver2024_17 {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<usize>,
    pointer: usize,
    output: Vec<usize>,
}

impl Default for Solver2024_17 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2024_17 {
    fn from(input: &str) -> Self {
        let re_register_a = Regex::new(r"Register A: (\d+)").unwrap();
        let re_register_b = Regex::new(r"Register B: (\d+)").unwrap();
        let re_register_c = Regex::new(r"Register C: (\d+)").unwrap();
        let re_program = Regex::new(r"Program: (.*)").unwrap();
        Self {
            register_a: re_register_a
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            register_b: re_register_b
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            register_c: re_register_c
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            program: re_program
                .captures(input)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect(),
            pointer: 0,
            output: vec![],
        }
    }
}

impl Solver2024_17 {
    fn get_combo_operand(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => unreachable!(),
        }
    }

    fn apply_operation(&mut self) {
        let operator = self.program[self.pointer];
        let operand = self.program[self.pointer + 1];
        self.pointer += 2;
        match operator {
            0 => {
                self.register_a /= 2_usize.pow(self.get_combo_operand(operand) as u32);
            }
            1 => {
                self.register_b ^= operand;
            }
            2 => {
                self.register_b = self.get_combo_operand(operand) % 8;
            }
            3 => {
                if self.register_a != 0 {
                    self.pointer = operand;
                }
            }
            4 => {
                self.register_b ^= self.register_c;
            }
            5 => {
                self.output.push(self.get_combo_operand(operand) % 8);
            }
            6 => {
                self.register_b =
                    self.register_a / 2_usize.pow(self.get_combo_operand(operand) as u32);
            }
            7 => {
                self.register_c =
                    self.register_a / 2_usize.pow(self.get_combo_operand(operand) as u32);
            }
            _ => unreachable!(),
        }
    }
    fn run(&mut self) {
        while self.program.len() > (self.pointer + 1) {
            self.apply_operation();
        }
    }

    fn run_until_first_output(&mut self) {
        while self.output.len() == 0 {
            self.apply_operation();
        }
    }
}

impl Solver<String, usize> for Solver2024_17 {
    fn solve_first_part(&self) -> String {
        let mut mutated = self.clone();
        mutated.run();
        mutated
            .output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn solve_second_part(&self) -> usize {
        let mut candidates: Vec<usize> = (0..8).collect();
        let mut next_candidates: Vec<usize> = vec![];
        for i in (0..self.program.len()).rev() {
            for candidate in candidates.iter() {
                let mut mutated = self.clone();
                mutated.register_a = *candidate;
                mutated.run_until_first_output();
                if mutated.output[0] == self.program[i] {
                    let n = candidate * 8;
                    if i == 0 {
                        next_candidates.push(*candidate);
                    } else {
                        next_candidates.extend(n..n + 8);
                    }
                }
            }
            candidates = next_candidates.clone();
            next_candidates = vec![];
        }

        candidates.into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_first_part() {
        let solver = Solver2024_17::from(
            "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0\
",
        );

        assert_eq!(solver.solve_first_part(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_solve_second_part() {
        let solver = Solver2024_17::from(
            "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0\
",
        );
        assert_eq!(solver.solve_second_part(), 117440);
    }
}
