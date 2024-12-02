use std::collections::HashMap;
use crate::solver::utils;

fn get_lists() -> (Vec<i32>, Vec<i32>) {
    let input = utils::get_input("inputs/2024_01.txt");

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in input.lines() {
        let mut line = line.split_whitespace();
        let left = line.next().unwrap().parse().unwrap();
        let right = line.next().unwrap().parse().unwrap();
        left_list.push(left);
        right_list.push(right);
    }

    (left_list, right_list)
}

fn solve_first_part(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    let mut left_list = left_list.clone();
    left_list.sort();
    let mut right_list = right_list.clone();
    right_list.sort();

    left_list.iter().enumerate().fold(0, |acc, (index, left)| {
        acc + (left - right_list[index]).abs()
    })
}
fn solve_second_part(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    let count_map = right_list.iter().fold(HashMap::new(), |mut map, right| {
        map.entry(right).and_modify(|count| *count += 1).or_insert(1);
        map
    });
    left_list.iter().fold(0, |acc, left| {
        acc + (count_map.get(left).unwrap_or(&0) * left)
    })
}

pub fn solve() -> (i32, i32) {
    let (left_list, right_list) = get_lists();
    (
        solve_first_part(&left_list, &right_list),
        solve_second_part(&left_list, &right_list),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_first_part() {
        assert_eq!(
            super::solve_first_part(&vec![3, 4, 2, 1, 3, 3, ], &vec![4, 3, 5, 3, 9, 3, ], ),
            11
        );
    }
    #[test]
    fn solve_second_part() {
        assert_eq!(
            super::solve_second_part(&vec![3, 4, 2, 1, 3, 3, ], &vec![4, 3, 5, 3, 9, 3, ], ),
            31
        );
    }
}
