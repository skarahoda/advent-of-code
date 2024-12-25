use super::Solver;
use regex::Regex;
use std::collections::HashMap;

enum Operation {
    And,
    Or,
    Xor,
}
pub struct Solver2024_24<'a> {
    initial_values: HashMap<&'a str, bool>,
    assignments: HashMap<&'a str, (&'a str, Operation, &'a str)>,
}

impl Default for Solver2024_24<'_> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2024_24<'a> {
    fn from(input: &'a str) -> Self {
        let re_initial_values = Regex::new("(.*): (0|1)").unwrap();
        let re_assignments = Regex::new("(.*) (AND|OR|XOR) (.*) -> (.*)").unwrap();
        let mut parts = input.split("\n\n");
        let initial_values = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let captures = re_initial_values.captures(line).unwrap();
                (
                    captures.get(1).unwrap().as_str(),
                    captures.get(2).unwrap().as_str() == "1",
                )
            })
            .collect();
        let assignments = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let captures = re_assignments.captures(line).unwrap();
                (
                    captures.get(4).unwrap().as_str(),
                    (
                        captures.get(1).unwrap().as_str(),
                        match captures.get(2).unwrap().as_str() {
                            "AND" => Operation::And,
                            "OR" => Operation::Or,
                            "XOR" => Operation::Xor,
                            _ => unreachable!(),
                        },
                        captures.get(3).unwrap().as_str(),
                    ),
                )
            })
            .collect();
        Self {
            initial_values,
            assignments,
        }
    }
}

impl<'a> Solver2024_24<'a> {
    fn get_value(&'a self, values: &mut HashMap<&'a str, bool>, variable: &'a str) -> bool {
        if let Some(value) = values.get(variable) {
            return *value;
        }
        let (lhs, operation, rhs) = self.assignments.get(variable).unwrap();
        let value = match operation {
            Operation::And => self.get_value(values, lhs) && self.get_value(values, rhs),
            Operation::Or => self.get_value(values, lhs) || self.get_value(values, rhs),
            Operation::Xor => self.get_value(values, lhs) ^ self.get_value(values, rhs),
        };
        values.insert(variable, value);
        value
    }
    fn get_values(&'a self) -> HashMap<&'a str, bool> {
        let mut result = self.initial_values.clone();
        for variable in self.assignments.keys() {
            self.get_value(&mut result, variable);
        }
        result
    }

    fn get_z_value(&self) -> usize {
        let mut result = 0;
        let values = self.get_values();
        for i in 0..99 {
            if let Some(&value) = values.get(&format!("z{:0>2}", i).as_str()) {
                result |= (value as usize) << i;
            } else {
                break;
            }
        }
        result
    }
}

impl Solver<usize, usize> for Solver2024_24<'_> {
    fn solve_first_part(&self) -> usize {
        self.get_z_value()
    }

    fn solve_second_part(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static EXAMPLE: &str = include_str!("example.txt");
    static EXAMPLE2: &str = include_str!("example2.txt");

    #[test]
    fn test_solve_first_part() {
        let solver = Solver2024_24::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 4);
    }

    #[test]
    fn test_solve_first_part_with_example2() {
        let solver = Solver2024_24::from(EXAMPLE2);
        assert_eq!(solver.solve_first_part(), 2024);
    }

    #[test]
    fn test_solve_second_part() {
        let solver = Solver2024_24::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 0);
    }
}
