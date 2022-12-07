use regex::Regex;
use super::utils;

fn does_not_contain_forbidden_words(input: &str) -> bool {
    !input.contains("ab")
        && !input.contains("cd")
        && !input.contains("pq")
        && !input.contains("xy")
}

fn does_contain_same_characters_in_a_row(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    for i in 1..chars.len() {
        if chars[i-1] == chars[i] {
            return true;
        }
    }
    false
}

fn does_contain_three_vowel(input: &str) -> bool {
    let re = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    re.is_match(input)
}

fn is_nice_string(input: &str) -> bool {
    does_contain_three_vowel(input)
        && does_not_contain_forbidden_words(input)
        && does_contain_same_characters_in_a_row(input)
}


fn does_contain_same_characters_with_one_space(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    for i in 2..chars.len() {
        if chars[i-2] == chars[i] {
            return true;
        }
    }
    false
}

fn does_contain_repeating_two_letters(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    for i in 2..chars.len() {
        let rest = &input[i..];
        let consecutive_characters = std::format!("{}{}",chars[i-2], chars[i-1]);
        if rest.contains(consecutive_characters.as_str()) {
            return true;
        }
    }
    false
}

fn solve_first_part(words: &Vec<&str>) -> usize {
    words.iter().filter(|word| is_nice_string(word)).count()
}
fn solve_second_part(words: &Vec<&str>) -> usize {
    words.iter()
        .filter(|word| {
            does_contain_same_characters_with_one_space(word)
                && does_contain_repeating_two_letters(word)
        }).count()
}

pub fn solve() -> (usize, usize) {
    let input = utils::get_input("inputs/2015_05.txt");
    let words = input.split("\n").collect();
    (
        solve_first_part(&words),
        solve_second_part(&words)
    )
}

#[cfg(test)]
mod does_contain_two_characters_in_a_row {
    #[test]
    fn when_input_have_same_characters_at_the_beginning() {
        assert_eq!(super::does_contain_same_characters_in_a_row("aaqwertyuiop"), true);
    }
    #[test]
    fn when_input_have_same_characters_at_the_end() {
        assert_eq!(super::does_contain_same_characters_in_a_row("qwertyuiobb"), true);
    }
    #[test]
    fn when_input_have_same_characters_at_the_middle() {
        assert_eq!(super::does_contain_same_characters_in_a_row("qwertyuiggxcvbnm"), true);
    }
    #[test]
    fn when_input_does_not_have_same_characters() {
        assert_eq!(super::does_contain_same_characters_in_a_row("asdfghjkl"), false);
    }
}

#[cfg(test)]
mod does_contain_vowel {
    #[test]
    fn when_input_have_three_vowel() {
        assert_eq!(super::does_contain_three_vowel("aaa"), true);
    }
    #[test]
    fn when_input_have_three_different_vowel() {
        assert_eq!(super::does_contain_three_vowel("dsmkvdsflfanjkfvkejdlkdsuvjkn"), true);
    }
    #[test]
    fn when_input_have_two_different_vowel() {
        assert_eq!(super::does_contain_three_vowel("djxxsnxbuhdjsdkdncxboxgsjxn"), false);
    }
    #[test]
    fn when_input_does_not_have_vowel() {
        assert_eq!(super::does_contain_three_vowel("qwrtypsdfghjklzxcvbnm"), false);
    }
}

#[cfg(test)]
mod is_nice_string {
    #[test]
    fn when_long_nice_string() {
        assert_eq!(super::is_nice_string("ugknbfddgicrmopn"), true);
    }
    #[test]
    fn when_short_nice_string() {
        assert_eq!(super::is_nice_string("aaa"), true);
    }
    #[test]
    fn when_no_same_characters_in_a_row() {
        assert_eq!(super::is_nice_string("jchzalrnumimnmhp"), false);
    }
    #[test]
    fn when_have_forbidden_words() {
        assert_eq!(super::is_nice_string("haegwjzuvuyypxyu"), false);
    }
    #[test]
    fn second_example() {
        assert_eq!(super::is_nice_string("dvszwmarrgswjxmb"), false);
    }
}

#[cfg(test)]
mod does_contain_repeating_two_letters {
    #[test]
    fn when_repeating_character_at_the_beginning() {
        assert_eq!(super::does_contain_repeating_two_letters("abab"), true);
    }
    #[test]
    fn when_repeating_character_at_the_beginning_and_end() {
        assert_eq!(super::does_contain_repeating_two_letters("aafghjmnnjjjaa"), true);
    }
    #[test]
    fn when_no_repeating_characters() {
        assert_eq!(super::does_contain_repeating_two_letters("aaa"), false);
    }
}
