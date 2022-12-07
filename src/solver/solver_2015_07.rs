use std::collections::HashMap;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

#[derive(Parser)]
#[grammar = "solver/solver_2015_07.pest"]
struct SantaParser;

fn can_evaluate(pairs: Vec<Pair<Rule>>) -> bool {
    (pairs.len() == 1 && pairs[0].as_rule() == Rule::number)
    || (pairs.len() == 2 && pairs[1].as_rule() == Rule::number)
    || (pairs.len() == 3 && pairs[0].as_rule() == Rule::number && pairs[2].as_rule() == Rule::number)
}

fn solve_first_part(input: &str) -> usize {
    let pairs = SantaParser::parse(Rule::program, input).unwrap_or_else(|e| panic!("{}", e));
    let program = pairs.peek().unwrap();
    let mut rule_map = HashMap::<&str, Vec<Pair<Rule>>>::new();
    let mut dependency_map = HashMap::<&str, Vec<&str>>::new();
    for statement in program.into_inner() {
        let rules: Vec<Pair<Rule>> = statement.into_inner().collect();
        let rhs = rules[1].as_str();
        rule_map.insert(rhs, rules[0].clone().into_inner().collect());
        for rule in rules[0].clone().into_inner() {
            match rule.as_rule() {
                Rule::ident => dependency_map.entry(rhs)
                    .or_insert(Vec::new())
                    .push(rule.as_str()),
                _ => ()
            }
        }
    }
    let queue =
    for (rhs, lhs) in rule_map.into_iter() {

    }
    dbg!(rule_map, dependency_map);
    13231
}

fn solve_second_part() -> usize {
    13123
}

pub fn solve() -> (usize, usize) {
    (
        solve_first_part("asdsa"),
        solve_second_part()
    )
}


#[cfg(test)]
mod find_start_of_message_marker {
    static EXAMPLE: &str = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";
    #[test]
    fn should_solve_example() {
        assert_eq!(super::solve_first_part(EXAMPLE), 7);
    }
}

