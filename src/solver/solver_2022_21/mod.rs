mod input;
use input::INPUT;
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;

#[derive(pest_derive::Parser)]
#[grammar = "solver/solver_2022_21/grammar.pest"]
struct SantaParser;

type RuleMap<'a> = HashMap<&'a str, Vec<Pair<'a, Rule>>>;

struct Analyzer<'a> {
    rule_map: RuleMap<'a>,
    value_map: HashMap<&'a str, usize>,
}

impl<'a> Analyzer<'a> {
    pub fn new(input: &'a str) -> Self {
        let pairs = SantaParser::parse(Rule::Program, input).unwrap_or_else(|e| panic!("{}", e));
        let program = pairs.peek().unwrap();
        let mut rule_map = HashMap::<&str, Vec<Pair<Rule>>>::new();
        for statement in program.into_inner() {
            let rules: Vec<Pair<Rule>> = statement.into_inner().collect();
            let lhs = rules[0].as_str();
            rule_map.insert(lhs, rules[1].clone().into_inner().collect());
        }
        Self {
            rule_map,
            value_map: HashMap::new(),
        }
    }

    pub fn evaluate_ident(self: &'a Analyzer<'a>, variable: &'a str) -> Option<usize> {
        match self.value_map.get(variable) {
            Some(result) => Some(*result),
            None => {
                let pairs = self.rule_map.get(variable).unwrap();
                if pairs.len() == 1 {
                    pairs[0].as_str().parse().ok()
                } else if pairs.len() == 3 {
                    let left = self.value_map.get(pairs[0].as_str())?;
                    let right = self.value_map.get(pairs[2].as_str())?;
                    match pairs[1].as_rule() {
                        Rule::Add => Some(left + right),
                        Rule::Subtract => Some(left - right),
                        Rule::Multiply => Some(left * right),
                        Rule::Divide => Some(left / right),
                        other => panic!("syntax error: operation cannot be {:?}", other),
                    }
                } else {
                    panic!(
                        "syntax error: lhs cannot have more than three tokens. Found: {:?}",
                        pairs
                    )
                }
            }
        }
    }

    pub fn get_value(&mut self, variable: &'a str) -> usize {
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

    pub fn get_children(&self, parent: &'a str) -> (&'a str, &'a str) {
        let pairs = self.rule_map.get(parent).unwrap();
        (pairs[0].as_str(), pairs[2].as_str())
    }

    pub fn is_parent(&self, parent: &'a str, child: &'a str) -> bool {
        let pairs = self.rule_map.get(parent).unwrap();
        if parent == child {
            true
        } else if pairs.len() == 1 {
            false
        } else if pairs.len() == 3 {
            let (left, right) = self.get_children(parent);
            self.is_parent(left, child) || self.is_parent(right, child)
        } else {
            panic!(
                "syntax error: lhs cannot have more than three tokens. Found: {:?}",
                pairs
            )
        }
    }

    pub fn get_operator(&self, variable: &'a str) -> Rule {
        self.rule_map.get(variable).unwrap()[1].as_rule()
    }
}

fn solve_first_part(input: &str) -> usize {
    let mut analyzer = Analyzer::new(input);
    analyzer.get_value("root")
}

fn solve_second_part(input: &str) -> usize {
    let mut analyzer = Analyzer::new(input);
    let (left, right) = analyzer.get_children("root");
    let (mut human_parent, mut other_value) = if analyzer.is_parent(left, "humn") {
        (left, analyzer.get_value(right))
    } else {
        (right, analyzer.get_value(left))
    };
    let mut result = other_value;
    while human_parent != "humn" {
        let (left, right) = analyzer.get_children(human_parent);
        let operator = analyzer.get_operator(human_parent);
        (human_parent, other_value) = if analyzer.is_parent(left, "humn") {
            (left, analyzer.get_value(right))
        } else {
            (right, analyzer.get_value(left))
        };
        result = match operator {
            Rule::Add => result - other_value,
            Rule::Subtract => {
                if analyzer.is_parent(left, "humn") {
                    result + other_value
                } else {
                    other_value - result
                }
            }
            Rule::Multiply => result / other_value,
            Rule::Divide => {
                if analyzer.is_parent(left, "humn") {
                    result * other_value
                } else {
                    other_value / result
                }
            }
            other => panic!("syntax error: operation cannot be {:?}", other),
        };
    }
    result
}

pub fn solve() -> (usize, usize) {
    (solve_first_part(INPUT), solve_second_part(INPUT))
}

#[cfg(test)]
mod find_start_of_message_marker {
    static EXAMPLE: &str = "\
        root: pppw + sjmn\n\
        dbpl: 5\n\
        cczh: sllz + lgvd\n\
        zczc: 2\n\
        ptdq: humn - dvpt\n\
        dvpt: 3\n\
        lfqf: 4\n\
        humn: 5\n\
        ljgn: 2\n\
        sjmn: drzm * dbpl\n\
        sllz: 4\n\
        pppw: cczh / lfqf\n\
        lgvd: ljgn * ptdq\n\
        drzm: hmdt - zczc\n\
        hmdt: 32\
    ";

    #[test]
    fn should_solve_first_part() {
        assert_eq!(super::solve_first_part(EXAMPLE), 152);
    }

    #[test]
    fn should_solve_second_part() {
        assert_eq!(super::solve_second_part(EXAMPLE), 301);
    }
}
