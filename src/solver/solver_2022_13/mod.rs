use super::Solver;
mod input;
use input::INPUT;
use pest::iterators::Pair;
use pest::Parser;
use std::cmp::Ordering;

#[derive(pest_derive::Parser)]
#[grammar = "solver/solver_2022_13/grammar.pest"]
struct SantaParser;
type List = Vec<Box<ListItem>>;

fn cmp(left: &List, right: &List) -> Ordering {
    if left.is_empty() && right.is_empty() {
        Ordering::Equal
    } else if left.is_empty() {
        Ordering::Less
    } else if right.is_empty() {
        Ordering::Greater
    } else if left[0] == right[0] {
        left[1..].cmp(&right[1..])
    } else {
        left[0].cmp(&right[0])
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum ListItem {
    Integer(u32),
    List(List),
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.clone(), other.clone()) {
            (ListItem::Integer(left), ListItem::Integer(right)) => left.cmp(&right),
            (ListItem::List(left), ListItem::List(right)) => cmp(&left, &right),
            (left, ListItem::List(right)) => cmp(&vec![Box::new(left)], &right),
            (ListItem::List(left), right) => cmp(&left, &vec![Box::new(right)]),
        }
    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solver2022_13 {
    pair_of_packets: Vec<(List, List)>,
}

impl Default for Solver2022_13 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

fn parse_list(list: Pair<Rule>) -> List {
    list.into_inner()
        .map(|item| {
            Box::new(match item.as_rule() {
                Rule::Number => ListItem::Integer(item.as_str().parse().unwrap()),
                Rule::List => ListItem::List(parse_list(item)),
                other => panic!("syntax error: list item expected, found {:?}", other),
            })
        })
        .collect()
}

impl From<&str> for Solver2022_13 {
    fn from(input: &str) -> Self {
        let packets = SantaParser::parse(Rule::Packets, input).unwrap_or_else(|e| panic!("{}", e));
        let pair_of_packets = packets
            .peek()
            .unwrap()
            .into_inner()
            .map(|packet| {
                let mut pair = packet.into_inner();
                (
                    parse_list(pair.next().unwrap()),
                    parse_list(pair.next().unwrap()),
                )
            })
            .collect();
        Self { pair_of_packets }
    }
}

impl Solver<usize, usize> for Solver2022_13 {
    fn solve_first_part(&self) -> usize {
        self.pair_of_packets
            .iter()
            .enumerate()
            .fold(
                0,
                |sum, (i, (left, right))| if left <= right { sum + i + 1 } else { sum },
            )
    }

    fn solve_second_part(&self) -> usize {
        let first: &List = &vec![Box::new(ListItem::List(vec![Box::new(ListItem::Integer(
            2,
        ))]))];
        let second: &List = &vec![Box::new(ListItem::List(vec![Box::new(ListItem::Integer(
            6,
        ))]))];

        let mut packets: Vec<List> = self.pair_of_packets.iter().fold(
            vec![first.clone(), second.clone()],
            |mut vector, (left, right)| {
                vector.push(left.clone());
                vector.push(right.clone());
                vector
            },
        );
        packets.sort();
        let (first_index, _) = packets
            .iter()
            .enumerate()
            .find(|&(_, list)| cmp(list, first) == Ordering::Equal)
            .unwrap();
        let (second_index, _) = packets
            .iter()
            .enumerate()
            .find(|&(_, list)| cmp(list, second) == Ordering::Equal)
            .unwrap();
        (first_index + 1) * (second_index + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
        [1,1,3,1,1]\n\
        [1,1,5,1,1]\n\
        \n\
        [[1],[2,3,4]]\n\
        [[1],4]\n\
        \n\
        [9]\n\
        [[8,7,6]]\n\
        \n\
        [[4,4],4,4]\n\
        [[4,4],4,4,4]\n\
        \n\
        [7,7,7,7]\n\
        [7,7,7]\n\
        \n\
        []\n\
        [3]\n\
        \n\
        [[[]]]\n\
        [[]]\n\
        \n\
        [1,[2,[3,[4,[5,6,7]]]],8,9]\n\
        [1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn should_parse_input() {
        let solver = Solver2022_13::from(EXAMPLE);
        assert_eq!(
            solver.pair_of_packets[0..=1],
            vec![
                (
                    vec![
                        Box::new(ListItem::Integer(1)),
                        Box::new(ListItem::Integer(1)),
                        Box::new(ListItem::Integer(3)),
                        Box::new(ListItem::Integer(1)),
                        Box::new(ListItem::Integer(1)),
                    ],
                    vec![
                        Box::new(ListItem::Integer(1)),
                        Box::new(ListItem::Integer(1)),
                        Box::new(ListItem::Integer(5)),
                        Box::new(ListItem::Integer(1)),
                        Box::new(ListItem::Integer(1)),
                    ]
                ),
                (
                    vec![
                        Box::new(ListItem::List(vec![Box::new(ListItem::Integer(1))])),
                        Box::new(ListItem::List(vec![
                            Box::new(ListItem::Integer(2)),
                            Box::new(ListItem::Integer(3)),
                            Box::new(ListItem::Integer(4)),
                        ])),
                    ],
                    vec![
                        Box::new(ListItem::List(vec![Box::new(ListItem::Integer(1))])),
                        Box::new(ListItem::Integer(4))
                    ]
                )
            ]
        )
    }

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2022_13::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 13);
    }
    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2022_13::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 140);
    }
}
