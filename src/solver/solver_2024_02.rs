use super::utils;

fn get_lists() -> Vec<Vec<i32>> {
    let input = utils::get_input("inputs/2024_02.txt");
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}


fn is_safe_list(list: &Vec<i32>) -> bool {
    let is_descending = list[0] < list[1];
    for i in 1..list.len() {
        let current = list[i];
        let previous = list[i - 1];
        if previous == current
            || (current - previous).abs() > 3
            || (is_descending && current < previous)
            || (!is_descending && current > previous) {
            return false;
        }
    }
    true
}

fn is_safe_list_with_a_removed_level(list: &Vec<i32>) -> bool {
    let is_descending = list[0] < list[1];
    for i in 1..list.len() {
        let current = list[i];
        let previous = list[i - 1];
        if previous == current {
            let mut list_without_current = list.clone();
            list_without_current.remove(i);
            return is_safe_list(&list_without_current);
        }
        if (current - previous).abs() > 3 {
            let mut list_without_previous = list.clone();
            list_without_previous.remove(i - 1);
            let mut list_without_current = list.clone();
            list_without_current.remove(i);
            return is_safe_list(&list_without_current) || is_safe_list(&list_without_previous);
        }
        if (is_descending && current < previous)
            || (!is_descending && current > previous) {
            let mut list_without_first = list.clone();
            list_without_first.remove(0);
            let mut list_without_previous = list.clone();
            list_without_previous.remove(i - 1);
            let mut list_without_current = list.clone();
            list_without_current.remove(i);
            return is_safe_list(&list_without_first) || is_safe_list(&list_without_current) || is_safe_list(&list_without_previous);
        }
    }
    true
}

fn solve_first_part(lists: &Vec<Vec<i32>>) -> i32 {
    lists.iter().fold(0, |acc, list| {
        if is_safe_list(list) {
            acc + 1
        } else {
            acc
        }
    })
}
fn solve_second_part(lists: &Vec<Vec<i32>>) -> i32 {
    lists.iter().fold(0, |acc, list| {
        if is_safe_list_with_a_removed_level(list) {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn solve() -> (i32, i32) {
    let lists = get_lists();
    (
        solve_first_part(&lists),
        solve_second_part(&lists),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn decide_unsafe_list_when_first_two_are_same() {
        assert_eq!(super::is_safe_list(&vec![10, 10]), false);
    }
    #[test]
    fn decide_safe_list_when_first_two_are_one_above_each_other() {
        assert_eq!(super::is_safe_list(&vec![1, 2]), true);
    }

    #[test]
    fn decide_safe_list_when_first_two_are_two_above_each_other() {
        assert_eq!(super::is_safe_list(&vec![1, 3]), true);
    }

    #[test]
    fn decide_safe_list_when_first_two_are_three_above_each_other() {
        assert_eq!(super::is_safe_list(&vec![1, 4]), true);
    }

    #[test]
    fn decide_unsafe_list_when_first_two_are_ascending_and_then_descending() {
        assert_eq!(super::is_safe_list(&vec![1, 2, 1]), false);
    }

    #[test]
    fn decide_unsafe_list_when_first_two_are_descending_and_then_ascending() {
        assert_eq!(super::is_safe_list(&vec![2, 1, 2]), false);
    }


    #[test]
    fn solve_first_part() {
        assert_eq!(
            super::solve_first_part(&vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]),
            2
        );
    }

    #[test]
    fn solve_second_part() {
        assert_eq!(
            super::solve_second_part(&vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]),
            4
        );
    }
}
