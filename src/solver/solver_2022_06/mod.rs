use super::Solver;
use std::borrow::Cow;
use std::collections::HashSet;

pub struct Solver2022_06<'a> {
    input: Cow<'a, str>,
}

impl<'a> Default for Solver2022_06<'a> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2022_06<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            input: Cow::Borrowed(input),
        }
    }
}

impl<'a> Solver2022_06<'a> {
    fn find_start_of_message_marker(&self, message_length: usize) -> usize {
        let chars: Vec<char> = self.input.chars().collect();
        for i in message_length - 1..chars.len() {
            let mut set = HashSet::<char>::new();
            for j in 0..message_length {
                set.insert(chars[i - j]);
            }
            if set.len() == message_length {
                return i + 1;
            }
        }
        panic!("couldn't solve it.");
    }
}

impl<'a> Solver<usize, usize> for Solver2022_06<'a> {
    fn solve_first_part(&self) -> usize {
        self.find_start_of_message_marker(4)
    }

    fn solve_second_part(&self) -> usize {
        self.find_start_of_message_marker(14)
    }
}

#[cfg(test)]
mod find_start_of_message_marker {
    use super::Solver;

    #[test]
    fn first_example() {
        let solver = super::Solver2022_06::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(solver.solve_first_part(), 7);
    }
    #[test]
    fn second_example() {
        let solver = super::Solver2022_06::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(solver.solve_first_part(), 5);
    }
    #[test]
    fn third_example() {
        let solver = super::Solver2022_06::from("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(solver.solve_first_part(), 6);
    }
    #[test]
    fn fourth_example() {
        let solver = super::Solver2022_06::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(solver.solve_first_part(), 10);
    }
    #[test]
    fn fifth_example() {
        let solver = super::Solver2022_06::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(solver.solve_first_part(), 11);
    }
    #[test]
    fn first_example_in_second_part() {
        let solver = super::Solver2022_06::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(solver.solve_second_part(), 19);
    }
    #[test]
    fn second_example_in_second_part() {
        let solver = super::Solver2022_06::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(solver.solve_second_part(), 23);
    }
    #[test]
    fn third_example_in_second_part() {
        let solver = super::Solver2022_06::from("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(solver.solve_second_part(), 23);
    }
    #[test]
    fn fourth_example_in_second_part() {
        let solver = super::Solver2022_06::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(solver.solve_second_part(), 29);
    }
    #[test]
    fn fifth_example_in_second_part() {
        let solver = super::Solver2022_06::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(solver.solve_second_part(), 26);
    }
}
