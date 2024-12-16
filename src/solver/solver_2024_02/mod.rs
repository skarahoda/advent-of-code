use super::Solver;

fn is_safe_list(list: &[i32]) -> bool {
    let is_descending = list[0] < list[1];
    for i in 1..list.len() {
        let current = list[i];
        let previous = list[i - 1];
        if previous == current
            || (current - previous).abs() > 3
            || (is_descending && current < previous)
            || (!is_descending && current > previous)
        {
            return false;
        }
    }
    true
}

fn is_safe_list_with_a_removed_level(list: &[i32]) -> bool {
    let is_descending = list[0] < list[1];
    for i in 1..list.len() {
        let current = list[i];
        let previous = list[i - 1];
        if previous == current {
            let mut list_without_current = list.to_vec();
            list_without_current.remove(i);
            return is_safe_list(&list_without_current);
        }
        if (current - previous).abs() > 3 {
            let mut list_without_previous = list.to_vec();
            list_without_previous.remove(i - 1);
            let mut list_without_current = list.to_vec();
            list_without_current.remove(i);
            return is_safe_list(&list_without_current) || is_safe_list(&list_without_previous);
        }
        if (is_descending && current < previous) || (!is_descending && current > previous) {
            let mut list_without_first = list.to_vec();
            list_without_first.remove(0);
            let mut list_without_previous = list.to_vec();
            list_without_previous.remove(i - 1);
            let mut list_without_current = list.to_vec();
            list_without_current.remove(i);
            return is_safe_list(&list_without_first)
                || is_safe_list(&list_without_current)
                || is_safe_list(&list_without_previous);
        }
    }
    true
}

pub struct Solver2024_02 {
    lists: Vec<Vec<i32>>,
}

impl Default for Solver2024_02 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2024_02 {
    fn from(input: &str) -> Self {
        let lists = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect();
        Self { lists }
    }
}

impl Solver<i32, i32> for Solver2024_02 {
    fn solve_first_part(&self) -> i32 {
        self.lists.iter().filter(|list| is_safe_list(list)).count() as i32
    }

    fn solve_second_part(&self) -> i32 {
        self.lists
            .iter()
            .filter(|list| is_safe_list_with_a_removed_level(list))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    #[test]
    fn decide_unsafe_list_when_first_two_are_same() {
        assert_eq!(is_safe_list(&[10, 10]), false);
    }

    #[test]
    fn decide_safe_list_when_first_two_are_one_above_each_other() {
        assert_eq!(is_safe_list(&[1, 2]), true);
    }

    #[test]
    fn decide_safe_list_when_first_two_are_two_above_each_other() {
        assert_eq!(is_safe_list(&[1, 3]), true);
    }

    #[test]
    fn decide_safe_list_when_first_two_are_three_above_each_other() {
        assert_eq!(is_safe_list(&[1, 4]), true);
    }

    #[test]
    fn decide_unsafe_list_when_first_two_are_ascending_and_then_descending() {
        assert_eq!(is_safe_list(&[1, 2, 1]), false);
    }

    #[test]
    fn decide_unsafe_list_when_first_two_are_descending_and_then_ascending() {
        assert_eq!(is_safe_list(&[2, 1, 2]), false);
    }

    #[test]
    fn solve_first_part() {
        let solver = Solver2024_02::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 2);
    }

    #[test]
    fn solve_second_part() {
        let solver = Solver2024_02::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 4);
    }
}
