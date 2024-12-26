use super::Solver;
use regex::Regex;
use std::collections::HashMap;

#[derive(PartialEq)]
enum GateType {
    And,
    Or,
    Xor,
}
pub struct Solver2024_24<'a> {
    initial_values: HashMap<&'a str, bool>,
    gates: HashMap<&'a str, (&'a str, GateType, &'a str)>,
}

impl Default for Solver2024_24<'_> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2024_24<'a> {
    fn from(input: &'a str) -> Self {
        let re_initial_values = Regex::new("(.*): (0|1)").unwrap();
        let re_gates = Regex::new("(.*) (AND|OR|XOR) (.*) -> (.*)").unwrap();
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
        let gates = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let captures = re_gates.captures(line).unwrap();
                (
                    captures.get(4).unwrap().as_str(),
                    (
                        captures.get(1).unwrap().as_str(),
                        match captures.get(2).unwrap().as_str() {
                            "AND" => GateType::And,
                            "OR" => GateType::Or,
                            "XOR" => GateType::Xor,
                            _ => unreachable!(),
                        },
                        captures.get(3).unwrap().as_str(),
                    ),
                )
            })
            .collect();
        Self {
            initial_values,
            gates,
        }
    }
}

fn get_bit_index(variable: &str) -> Option<usize> {
    let re = Regex::new(r"(x|y)(\d+)").unwrap();
    re.captures(variable)?
        .get(2)?
        .as_str()
        .parse::<usize>()
        .ok()
}

impl<'a> Solver2024_24<'a> {
    fn get_value(&'a self, values: &mut HashMap<&'a str, bool>, variable: &'a str) -> bool {
        if let Some(value) = values.get(variable) {
            return *value;
        }
        let (lhs, gate_type, rhs) = self.gates.get(variable).unwrap();
        let value = match gate_type {
            GateType::And => self.get_value(values, lhs) && self.get_value(values, rhs),
            GateType::Or => self.get_value(values, lhs) || self.get_value(values, rhs),
            GateType::Xor => self.get_value(values, lhs) ^ self.get_value(values, rhs),
        };
        values.insert(variable, value);
        value
    }
    fn get_values(&'a self) -> HashMap<&'a str, bool> {
        let mut result = self.initial_values.clone();
        for variable in self.gates.keys() {
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

    fn get_xor_input_variables(&self) -> HashMap<usize, &'a str> {
        let mut result: HashMap<usize, &'a str> = HashMap::new();
        for (variable, (rhs, gate_type, lhs)) in self.gates.iter() {
            if *gate_type != GateType::Xor {
                continue;
            }
            let rhs_index = get_bit_index(rhs);
            let lhs_index = get_bit_index(lhs);
            if rhs_index != lhs_index {
                panic!("variables {} and {} are not the same", rhs, lhs);
            } else if let Some(index) = rhs_index {
                result.insert(index, variable);
            }
        }
        result
    }

    fn is_nth_output_correct(
        &self,
        n: usize,
        xor_input_variables: &HashMap<usize, &'a str>,
    ) -> bool {
        let &xor_input_variable = xor_input_variables.get(&n).unwrap();
        if n == 0 {
            return xor_input_variable == "z00";
        }

        let (rhs, gate_type, lhs) = self.gates.get(format!("z{:0>2}", n).as_str()).unwrap();
        *gate_type == GateType::Xor && vec![*rhs, *lhs].contains(&xor_input_variable)
    }

    fn detect_tangled_variables(&self) -> Vec<String> {
        let mut result = vec![];
        let xor_input_variables = self.get_xor_input_variables();
        for (&n, &variable) in xor_input_variables.iter() {
            let output = format!("z{:0>2}", n);
            if !self.is_nth_output_correct(n, &xor_input_variables) {
                let (rhs, gate_type, lhs) = self.gates.get(output.as_str()).unwrap();
                if *gate_type != GateType::Xor {
                    // if the gate is not an xor gate, then the output is incorrect
                    // We need to swap with a wire
                    // Swapped wire will be found in the next iteration
                    result.push(output.clone());
                } else {
                    // If the gate is an xor gate, then one of the wires is incorrect
                    // Incorrect wire is found by checking the gate type
                    // the wire with the or gate is the correct wire
                    // Swap the other wire with xor_input_variable
                    result.push(variable.to_string());
                    let (_, gate_type, _) = self.gates.get(lhs).unwrap();
                    if *gate_type == GateType::Or {
                        result.push(rhs.to_string());
                    } else {
                        result.push(lhs.to_string());
                    }
                }
            }
        }

        for (variable, (rhs, gate_type, lhs)) in self.gates.iter() {
            if variable.starts_with("z") {
                continue;
            }
            let rhs_index = get_bit_index(rhs);
            let lhs_index = get_bit_index(lhs);
            if *gate_type == GateType::Xor && (rhs_index.is_none() || rhs_index != lhs_index) {
                result.push(variable.to_string());
            }
        }

        result
    }
}

impl Solver<usize, String> for Solver2024_24<'_> {
    fn solve_first_part(&self) -> usize {
        self.get_z_value()
    }

    fn solve_second_part(&self) -> String {
        let mut result = self.detect_tangled_variables();
        result.sort();
        result.join(",")
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
}
