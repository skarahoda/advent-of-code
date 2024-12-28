use super::Solver;

pub struct Solver2015_10<'a> {
    input: &'a str,
}

impl Default for Solver2015_10<'_> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2015_10<'a> {
    fn from(input: &'a str) -> Self {
        Self { input }
    }
}

fn look_and_say(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars();
    let mut current = chars.next().unwrap();
    let mut count = 1;
    for c in chars {
        if c == current {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(current);
            current = c;
            count = 1;
        }
    }
    result.push_str(&count.to_string());
    result.push(current);
    result
}

impl<'a> Solver2015_10<'a> {
    fn apply_look_and_say_n_times(&self, n: usize) -> usize {
        let mut result = self.input.to_string();
        for _ in 0..n {
            result = look_and_say(&result);
        }
        result.len()
    }
}

impl Solver<usize, usize> for Solver2015_10<'_> {
    fn solve_first_part(&self) -> usize {
        self.apply_look_and_say_n_times(40)
    }

    fn solve_second_part(&self) -> usize {
        self.apply_look_and_say_n_times(50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_look_and_say_1() {
        assert_eq!(look_and_say("1"), "11");
    }

    #[test]
    fn should_look_and_say_11() {
        assert_eq!(look_and_say("11"), "21");
    }

    #[test]
    fn should_look_and_say_21() {
        assert_eq!(look_and_say("21"), "1211");
    }

    #[test]
    fn should_look_and_say_1211() {
        assert_eq!(look_and_say("1211"), "111221");
    }

    #[test]
    fn should_look_and_say_111221() {
        assert_eq!(look_and_say("111221"), "312211");
    }
}
