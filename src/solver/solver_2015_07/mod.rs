mod input;
use input::INPUT;
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;

#[derive(pest_derive::Parser)]
#[grammar = "solver/solver_2015_07/grammar.pest"]
struct SantaParser;

type RuleMap<'a> = HashMap<&'a str, Vec<Pair<'a, Rule>>>;

struct Analyzer<'a> {
    rule_map: RuleMap<'a>,
    value_map: HashMap<&'a str, u16>,
}

impl<'a> Analyzer<'a> {
    pub fn new(input: &'a str) -> Self {
        let pairs = SantaParser::parse(Rule::Program, input).unwrap_or_else(|e| panic!("{}", e));
        let program = pairs.peek().unwrap();
        let mut rule_map = HashMap::<&str, Vec<Pair<Rule>>>::new();
        for statement in program.into_inner() {
            let rules: Vec<Pair<Rule>> = statement.into_inner().collect();
            let rhs = rules[1].as_str();
            rule_map.insert(rhs, rules[0].clone().into_inner().collect());
        }
        Self {
            rule_map,
            value_map: HashMap::new(),
        }
    }

    fn evaluate_expr(&'a self, pair: &'a Pair<Rule>) -> Option<u16> {
        match pair.as_rule() {
            Rule::Ident => self.value_map.get(pair.as_str()).map(|v| *v),
            Rule::Number => Some(pair.as_str().parse().ok()?),
            other => panic!("syntax error: expr cannot be {:?}", other),
        }
    }

    pub fn evaluate_ident(self: &'a Analyzer<'a>, variable: &'a str) -> Option<u16> {
        match self.value_map.get(variable) {
            Some(result) => Some(*result),
            None => {
                let pairs = self.rule_map.get(variable).unwrap();
                let result = if pairs.len() == 1 {
                    self.evaluate_expr(&pairs[0])
                } else if pairs.len() == 2 {
                    Some(!self.evaluate_expr(&pairs[1])?)
                } else if pairs.len() == 3 {
                    match pairs[1].as_rule() {
                        Rule::And => {
                            Some(self.evaluate_expr(&pairs[0])? & self.evaluate_expr(&pairs[2])?)
                        }
                        Rule::Or => {
                            Some(self.evaluate_expr(&pairs[0])? | self.evaluate_expr(&pairs[2])?)
                        }
                        Rule::LShift => {
                            Some(self.evaluate_expr(&pairs[0])? << self.evaluate_expr(&pairs[2])?)
                        }
                        Rule::RShift => {
                            Some(self.evaluate_expr(&pairs[0])? >> self.evaluate_expr(&pairs[2])?)
                        }
                        other => panic!("syntax error: operation cannot be {:?}", other),
                    }
                } else {
                    panic!(
                        "syntax error: lhs cannot have more than three tokens. Found: {:?}",
                        pairs
                    )
                };
                result
            }
        }
    }

    pub fn get_value(&mut self, variable: &'a str) -> u16 {
        loop {
            for key in self.rule_map.keys() {
                match self.evaluate_ident(key) {
                    Some(value) => {
                        self.value_map.insert(key, value);
                    }
                    None => {}
                };
            }
            if let Some(value) = self.evaluate_ident(variable) {
                return value;
            }
        }
    }

    pub fn override_value(&mut self, variable: &'a str, value: u16) {
        self.value_map.clear();
        self.value_map.insert(variable, value);
    }
}

fn solve_first_part(input: &str, variable: &str) -> u16 {
    let mut analyzer = Analyzer::new(input);
    analyzer.get_value(variable)
}

fn solve_second_part(input: &str) -> u16 {
    let mut analyzer = Analyzer::new(input);
    let previous_value = analyzer.get_value("a");
    analyzer.override_value("b", previous_value);
    analyzer.get_value("a")
}

pub fn solve() -> (u16, u16) {
    (
        solve_first_part(INPUT, "a"),
        solve_second_part(INPUT),
    )
}

#[cfg(test)]
mod find_start_of_message_marker {
    static EXAMPLE: &str = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";
    #[test]
    fn should_solve_x_in_example() {
        assert_eq!(super::solve_first_part(EXAMPLE, "x"), 123);
    }
    #[test]
    fn should_solve_y_in_example() {
        assert_eq!(super::solve_first_part(EXAMPLE, "y"), 456);
    }
    #[test]
    fn should_solve_h_in_example() {
        assert_eq!(super::solve_first_part(EXAMPLE, "h"), 65412);
    }
    #[test]
    fn should_solve_i_in_example() {
        assert_eq!(super::solve_first_part(EXAMPLE, "i"), 65079);
    }
    #[test]
    fn should_solve_d_in_example() {
        assert_eq!(super::solve_first_part(EXAMPLE, "d"), 72);
    }
    #[test]
    fn should_solve_e_in_example() {
        assert_eq!(super::solve_first_part(EXAMPLE, "e"), 507);
    }
    #[test]
    fn should_solve_f_in_example() {
        assert_eq!(super::solve_first_part(EXAMPLE, "f"), 492);
    }
    #[test]
    fn should_solve_g_in_example() {
        assert_eq!(super::solve_first_part(EXAMPLE, "g"), 114);
    }
}
