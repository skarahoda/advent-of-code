use crate::solver::Solver;
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;

#[derive(pest_derive::Parser)]
#[grammar = "solver/solver_2015_07/grammar.pest"]
struct SantaParser;

pub struct Solver2015_07<'a> {
    rule_map: HashMap<&'a str, Vec<Pair<'a, Rule>>>,
    variable: &'a str,
}

impl Default for Solver2015_07<'_> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}
impl<'a> From<&'a str> for Solver2015_07<'a> {
    fn from(input: &'a str) -> Self {
        let pairs = SantaParser::parse(Rule::Program, input).unwrap();
        let program = pairs.peek().unwrap();
        let mut rule_map = HashMap::<&str, Vec<Pair<Rule>>>::new();
        for statement in program.into_inner() {
            let rules: Vec<Pair<Rule>> = statement.into_inner().collect();
            let rhs = rules[1].as_str();
            rule_map.insert(rhs, rules[0].clone().into_inner().collect());
        }
        Self {
            rule_map,
            variable: "a",
        }
    }
}

impl<'a> Solver2015_07<'a> {
    fn evaluate_expr(&'a self, pair: &'a Pair<Rule>, value_map: &mut HashMap<&'a str, u16>) -> u16 {
        match pair.as_rule() {
            Rule::Ident => self.evaluate_ident(pair.as_str(), value_map),
            Rule::Number => pair.as_str().parse().unwrap(),
            _ => unreachable!(),
        }
    }

    pub fn evaluate_ident(
        &'a self,
        variable: &'a str,
        value_map: &mut HashMap<&'a str, u16>,
    ) -> u16 {
        if let Some(&result) = value_map.get(variable) {
            return result;
        }
        let pairs = self.rule_map.get(variable).unwrap();
        let result = if pairs.len() == 1 {
            self.evaluate_expr(&pairs[0], value_map)
        } else if pairs.len() == 2 {
            !self.evaluate_expr(&pairs[1], value_map)
        } else if pairs.len() == 3 {
            match pairs[1].as_rule() {
                Rule::And => {
                    self.evaluate_expr(&pairs[0], value_map)
                        & self.evaluate_expr(&pairs[2], value_map)
                }
                Rule::Or => {
                    self.evaluate_expr(&pairs[0], value_map)
                        | self.evaluate_expr(&pairs[2], value_map)
                }
                Rule::LShift => {
                    self.evaluate_expr(&pairs[0], value_map)
                        << self.evaluate_expr(&pairs[2], value_map)
                }
                Rule::RShift => {
                    self.evaluate_expr(&pairs[0], value_map)
                        >> self.evaluate_expr(&pairs[2], value_map)
                }
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        };
        value_map.insert(variable, result);
        result
    }
}

impl Solver<u16, u16> for Solver2015_07<'_> {
    fn solve_first_part(&self) -> u16 {
        let mut value_map = HashMap::new();
        self.evaluate_ident(self.variable, &mut value_map)
    }
    fn solve_second_part(&self) -> u16 {
        let mut value_map = HashMap::new();
        let previous_value = self.evaluate_ident("a", &mut value_map);
        let mut value_map = HashMap::new();
        value_map.insert("b", previous_value);
        self.evaluate_ident("a", &mut value_map)
    }
}

#[cfg(test)]
mod find_start_of_message_marker {
    use super::*;
    static EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn should_solve_x_in_example() {
        let mut solver = Solver2015_07::from(EXAMPLE);
        solver.variable = "x";
        assert_eq!(solver.solve_first_part(), 123);
    }
    #[test]
    fn should_solve_y_in_example() {
        let mut solver = Solver2015_07::from(EXAMPLE);
        solver.variable = "y";
        assert_eq!(solver.solve_first_part(), 456);
    }
    #[test]
    fn should_solve_h_in_example() {
        let mut solver = Solver2015_07::from(EXAMPLE);
        solver.variable = "h";
        assert_eq!(solver.solve_first_part(), 65412);
    }
    #[test]
    fn should_solve_i_in_example() {
        let mut solver = Solver2015_07::from(EXAMPLE);
        solver.variable = "i";
        assert_eq!(solver.solve_first_part(), 65079);
    }
    #[test]
    fn should_solve_d_in_example() {
        let mut solver = Solver2015_07::from(EXAMPLE);
        solver.variable = "d";
        assert_eq!(solver.solve_first_part(), 72);
    }
    #[test]
    fn should_solve_e_in_example() {
        let mut solver = Solver2015_07::from(EXAMPLE);
        solver.variable = "e";
        assert_eq!(solver.solve_first_part(), 507);
    }
    #[test]
    fn should_solve_f_in_example() {
        let mut solver = Solver2015_07::from(EXAMPLE);
        solver.variable = "f";
        assert_eq!(solver.solve_first_part(), 492);
    }
    #[test]
    fn should_solve_g_in_example() {
        let mut solver = Solver2015_07::from(EXAMPLE);
        solver.variable = "g";
        assert_eq!(solver.solve_first_part(), 114);
    }
}
