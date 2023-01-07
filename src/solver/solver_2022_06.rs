use super::utils;
use std::collections::HashSet;

fn find_start_of_message_marker(input: &str, message_length: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
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

fn solve_first_part(input: &str) -> usize {
    find_start_of_message_marker(input, 4)
}

fn solve_second_part(input: &str) -> usize {
    find_start_of_message_marker(input, 14)
}

pub fn solve() -> (usize, usize) {
    let input = utils::get_input("inputs/2022_06.txt");
    (solve_first_part(&input[..]), solve_second_part(&input[..]))
}

#[cfg(test)]
mod find_start_of_message_marker {
    #[test]
    fn first_example() {
        assert_eq!(
            super::find_start_of_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
            7
        );
    }
    #[test]
    fn second_example() {
        assert_eq!(
            super::find_start_of_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4),
            5
        );
    }
    #[test]
    fn third_example() {
        assert_eq!(
            super::find_start_of_message_marker("nppdvjthqldpwncqszvftbrmjlhg", 4),
            6
        );
    }
    #[test]
    fn fourth_example() {
        assert_eq!(
            super::find_start_of_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
    }
    #[test]
    fn fifth_example() {
        assert_eq!(
            super::find_start_of_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            11
        );
    }
    #[test]
    fn first_example_in_second_part() {
        assert_eq!(
            super::find_start_of_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            19
        );
    }
    #[test]
    fn second_example_in_second_part() {
        assert_eq!(
            super::find_start_of_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14),
            23
        );
    }
    #[test]
    fn third_example_in_second_part() {
        assert_eq!(
            super::find_start_of_message_marker("nppdvjthqldpwncqszvftbrmjlhg", 14),
            23
        );
    }
    #[test]
    fn fourth_example_in_second_part() {
        assert_eq!(
            super::find_start_of_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
    }
    #[test]
    fn fifth_example_in_second_part() {
        assert_eq!(
            super::find_start_of_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }
}
