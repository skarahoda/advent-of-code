use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use super::utils;

fn parse_inputs(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    (
        parts[0]
            .lines()
            .map(|line| {
                let nums: Vec<&str> = line.split('|').collect();
                (nums[0].parse().unwrap(), nums[1].parse().unwrap())
            })
            .collect(),
        parts[1]
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect(),
    )
}

fn get_precedence_map(precedences: &Vec<(i32, i32)>) -> HashMap<i32, HashSet<i32>> {
    precedences
        .iter()
        .fold(HashMap::new(), |mut map, (x, y)| {
            map.entry(*x)
                .or_insert_with(HashSet::new)
                .insert(*y);
            map
        })
}

fn is_valid_print(print: &Vec<i32>, precedence_map: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut visited = HashSet::new();
    for &x in print {
        let precedences = precedence_map.get(&x);
        if precedences.unwrap_or(&HashSet::new()).iter().any(|y| visited.contains(y)) {
            return false;
        }
        visited.insert(x);
    }
    true
}

fn sort_print(print: &mut Vec<i32>, precedence_map: &HashMap<i32, HashSet<i32>>) {
    print.sort_by(|a, b| {
        let a_precedences = precedence_map.get(a);
        let b_precedences = precedence_map.get(b);
        if a_precedences.is_some() && a_precedences.unwrap().contains(b) {
            Ordering::Less
        } else if b_precedences.is_some() && b_precedences.unwrap().contains(a) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
}

fn get_middle<T>(array: &Vec<T>) -> Option<&T> {
    array.get((array.len() - 1) / 2)
}

fn solve_first_part(precedences: &Vec<(i32, i32)>, prints: &Vec<Vec<i32>>) -> i32 {
    let precedence_map = get_precedence_map(precedences);
    let mut result = 0;
    for print in prints {
        if is_valid_print(print, &precedence_map) {
            result += get_middle(print).unwrap();
        }
    }
    result
}
fn solve_second_part(precedences: &Vec<(i32, i32)>, prints: &Vec<Vec<i32>>) -> i32 {
    let precedence_map = get_precedence_map(precedences);
    prints.iter().fold(0, |acc, print| {
        if is_valid_print(print, &precedence_map) {
            return acc;
        }
        let mut print = print.clone();
        sort_print(&mut print, &precedence_map);
        acc + *get_middle(&print).unwrap()
    })
}

pub fn solve() -> (i32, i32) {
    let input = utils::get_input("inputs/2024_05.txt");
    let (precedences, prints) = parse_inputs(&input);
    (
        solve_first_part(&precedences, &prints),
        solve_second_part(&precedences, &prints),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_first_part() {
        let (precedences, prints) = super::parse_inputs(
            r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        );
        assert_eq!(
            super::solve_first_part(&precedences, &prints),
            143
        );
    }

    #[test]
    fn solve_second_part() {
        let (precedences, prints) = super::parse_inputs(
            r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,97,47,61,53
61,13,29
97,13,75,29,47"
        );
        assert_eq!(
            super::solve_second_part(&precedences, &prints),
            123
        );
    }
}
