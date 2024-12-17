use super::Solver;
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;

#[derive(pest_derive::Parser)]
#[grammar = "solver/solver_2022_11/grammar.pest"]
struct SantaParser;

#[derive(PartialEq, Debug, Clone)]
enum Operand {
    Old,
    Number(u64),
}

impl Operand {
    fn evaluate<'a>(&'a self, old: &'a u64) -> &'a u64 {
        match self {
            Operand::Old => old,
            Operand::Number(value) => value,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Operation {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
}

impl Operation {
    fn evaluate(&self, old: &u64) -> u64 {
        match self {
            Operation::Add(left, right) => left.evaluate(old) + right.evaluate(old),
            Operation::Multiply(left, right) => left.evaluate(old) * right.evaluate(old),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    tester: u64,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn new(
        items: Vec<u64>,
        operation: Operation,
        tester: u64,
        if_true: usize,
        if_false: usize,
    ) -> Self {
        Self {
            items,
            operation,
            tester,
            if_true,
            if_false,
        }
    }

    fn throw<F>(&self, item: &u64, relief: F) -> (usize, u64)
    where
        F: Fn(u64) -> u64,
    {
        let item = relief(self.operation.evaluate(item));
        let monkey_index = if item % self.tester == 0 {
            self.if_true
        } else {
            self.if_false
        };
        (monkey_index, item)
    }
}

pub struct Solver2022_11 {
    monkeys: HashMap<usize, Monkey>,
}

impl Default for Solver2022_11 {
    fn default() -> Self {
        Solver2022_11::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2022_11 {
    fn from(input: &str) -> Self {
        let program = SantaParser::parse(Rule::Program, input).unwrap_or_else(|e| panic!("{}", e));
        let monkey_pairs = program.peek().unwrap().into_inner();
        let mut monkeys = HashMap::new();

        for monkey_pair in monkey_pairs {
            let pairs: Vec<Pair<Rule>> = monkey_pair.into_inner().collect();
            let monkey_index: usize = pairs[0].as_str().parse().unwrap();
            let items: Vec<u64> = pairs[1]
                .to_owned()
                .into_inner()
                .map(|i| i.as_str().parse().unwrap())
                .collect();
            let left_operand = match pairs[2].as_rule() {
                Rule::Old => Operand::Old,
                Rule::Number => Operand::Number(pairs[2].as_str().parse().unwrap()),
                other => panic!("syntax error: operand expected, found {:?}", other),
            };
            let right_operand = match pairs[4].as_rule() {
                Rule::Old => Operand::Old,
                Rule::Number => Operand::Number(pairs[4].as_str().parse().unwrap()),
                other => panic!("syntax error: operand expected, found {:?}", other),
            };
            let operation = match pairs[3].as_rule() {
                Rule::Add => Operation::Add(left_operand, right_operand),
                Rule::Multiply => Operation::Multiply(left_operand, right_operand),
                other => panic!("syntax error: operator expected, found {:?}", other),
            };
            let tester: u64 = pairs[5].as_str().parse().unwrap();
            let if_true: usize = pairs[6].as_str().parse().unwrap();
            let if_false: usize = pairs[7].as_str().parse().unwrap();
            monkeys.insert(
                monkey_index,
                Monkey::new(items, operation, tester, if_true, if_false),
            );
        }

        Self { monkeys }
    }
}

impl Solver2022_11 {
    fn solve_part<F>(&self, rounds: usize, relief: F) -> u64
    where
        F: Fn(u64) -> u64,
    {
        let mut monkeys = self.monkeys.clone();
        let mut investigation_map: HashMap<usize, u64> = HashMap::new();

        for _ in 0..rounds {
            let mut new_items_map: HashMap<usize, Vec<u64>> = HashMap::new();
            for (index, monkey) in monkeys.iter() {
                for item in monkey.items.clone() {
                    let mut current_index = *index;
                    *investigation_map.entry(current_index).or_insert(0) += 1;
                    let (mut next_index, mut item) = monkey.throw(&item, &relief).clone();
                    while current_index <= next_index {
                        current_index = next_index;
                        *investigation_map.entry(current_index).or_insert(0) += 1;
                        (next_index, item) = monkeys
                            .get(&current_index)
                            .unwrap()
                            .throw(&item, &relief)
                            .clone();
                    }
                    new_items_map
                        .entry(next_index)
                        .or_insert(Vec::new())
                        .push(item);
                }
            }
            for monkey in monkeys.values_mut() {
                monkey.items.clear();
            }
            for (index, new_items) in new_items_map {
                for item in new_items {
                    monkeys.get_mut(&index).unwrap().items.push(item);
                }
            }
        }

        let mut investigations: Vec<&u64> = investigation_map.values().collect();
        investigations.sort();
        investigations.pop().unwrap() * investigations.pop().unwrap()
    }
}

impl Solver<u64, u64> for Solver2022_11 {
    fn solve_first_part(&self) -> u64 {
        self.solve_part(20, |x| x / 3)
    }

    fn solve_second_part(&self) -> u64 {
        let relief = self
            .monkeys
            .values()
            .fold(1, |product, monkey| product * monkey.tester);
        self.solve_part(10000, move |x| x % relief)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
        Monkey 0:\n\
        \x20\x20Starting items: 79, 98\n\
        \x20\x20Operation: new = old * 19\n\
        \x20\x20Test: divisible by 23\n\
        \x20\x20\x20\x20If true: throw to monkey 2\n\
        \x20\x20\x20\x20If false: throw to monkey 3\n\
        \n\
        Monkey 1:\n\
        \x20\x20Starting items: 54, 65, 75, 74\n\
        \x20\x20Operation: new = old + 6\n\
        \x20\x20Test: divisible by 19\n\
        \x20\x20\x20\x20If true: throw to monkey 2\n\
        \x20\x20\x20\x20If false: throw to monkey 0\n\
        \n\
        Monkey 2:\n\
        \x20\x20Starting items: 79, 60, 97\n\
        \x20\x20Operation: new = old * old\n\
        \x20\x20Test: divisible by 13\n\
        \x20\x20\x20\x20If true: throw to monkey 1\n\
        \x20\x20\x20\x20If false: throw to monkey 3\n\
        \n\
        Monkey 3:\n\
        \x20\x20Starting items: 74\n\
        \x20\x20Operation: new = old + 3\n\
        \x20\x20Test: divisible by 17\n\
        \x20\x20\x20\x20If true: throw to monkey 0\n\
        \x20\x20\x20\x20If false: throw to monkey 1";

    #[test]
    fn should_parse_input() {
        let solver = Solver2022_11::from(EXAMPLE);
        assert_eq!(
            solver.monkeys,
            HashMap::<usize, Monkey>::from([
                (
                    0,
                    Monkey::new(
                        vec![79, 98],
                        Operation::Multiply(Operand::Old, Operand::Number(19)),
                        23,
                        2,
                        3
                    )
                ),
                (
                    1,
                    Monkey::new(
                        vec![54, 65, 75, 74],
                        Operation::Add(Operand::Old, Operand::Number(6)),
                        19,
                        2,
                        0
                    )
                ),
                (
                    2,
                    Monkey::new(
                        vec![79, 60, 97],
                        Operation::Multiply(Operand::Old, Operand::Old),
                        13,
                        1,
                        3
                    )
                ),
                (
                    3,
                    Monkey::new(
                        vec![74],
                        Operation::Add(Operand::Old, Operand::Number(3)),
                        17,
                        0,
                        1
                    )
                ),
            ])
        )
    }

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2022_11::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 10605);
    }
    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2022_11::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 2713310158);
    }
}
