use crate::solver::Solver;
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;

#[derive(pest_derive::Parser)]
#[grammar = "solver/solver_2015_07/grammar.pest"]
struct SantaParser;

enum Expr<'a> {
    Ident(&'a str),
    Number(u16),
}

impl<'a> From<Pair<'a, Rule>> for Expr<'a> {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::Ident => Self::Ident(pair.as_str()),
            Rule::Number => Self::Number(pair.as_str().parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

enum Statement<'a> {
    And(Expr<'a>, Expr<'a>),
    Or(Expr<'a>, Expr<'a>),
    LShift(Expr<'a>, Expr<'a>),
    RShift(Expr<'a>, Expr<'a>),
    Not(Expr<'a>),
    Assign(Expr<'a>),
}

impl<'a> From<Pair<'a, Rule>> for Statement<'a> {
    fn from(pair: Pair<'a, Rule>) -> Self {
        let inner: Vec<Pair<Rule>> = pair.into_inner().collect();
        if inner.len() == 1 {
            Self::Assign(inner[0].clone().into())
        } else if inner.len() == 2 {
            Self::Not(inner[1].clone().into())
        } else if inner.len() == 3 {
            let lhs = inner[0].clone().into();
            let rhs = inner[2].clone().into();
            match inner[1].as_rule() {
                Rule::And => Self::And(lhs, rhs),
                Rule::Or => Self::Or(lhs, rhs),
                Rule::LShift => Self::LShift(lhs, rhs),
                Rule::RShift => Self::RShift(lhs, rhs),
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }
}

pub struct Solver2015_07<'a> {
    statements: HashMap<&'a str, Statement<'a>>,
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
        let mut statements = HashMap::<&str, Statement<'a>>::new();
        for statement in program.into_inner() {
            let rules: Vec<Pair<Rule>> = statement.into_inner().collect();
            let rhs = rules[1].as_str();
            statements.insert(rhs, rules[0].clone().into());
        }
        Self {
            statements,
            variable: "a",
        }
    }
}

impl<'a> Solver2015_07<'a> {
    fn evaluate_expr(&'a self, expr: &'a Expr, value_map: &mut HashMap<&'a str, u16>) -> u16 {
        match expr {
            Expr::Ident(variable) => self.evaluate_ident(variable, value_map),
            Expr::Number(number) => *number,
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
        let result = match self.statements.get(variable).unwrap() {
            Statement::Assign(exp) => self.evaluate_expr(exp, value_map),
            Statement::And(lhs, rhs) => {
                self.evaluate_expr(lhs, value_map) & self.evaluate_expr(rhs, value_map)
            }
            Statement::Or(lhs, rhs) => {
                self.evaluate_expr(lhs, value_map) | self.evaluate_expr(rhs, value_map)
            }
            Statement::LShift(lhs, rhs) => {
                self.evaluate_expr(lhs, value_map) << self.evaluate_expr(rhs, value_map)
            }
            Statement::RShift(lhs, rhs) => {
                self.evaluate_expr(lhs, value_map) >> self.evaluate_expr(rhs, value_map)
            }
            Statement::Not(expr) => !self.evaluate_expr(expr, value_map),
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
