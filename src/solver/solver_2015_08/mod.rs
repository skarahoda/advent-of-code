use super::Solver;

pub struct Solver2015_08 {
    strings: Vec<String>,
}

impl Default for Solver2015_08 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2015_08 {
    fn from(input: &str) -> Self {
        Self {
            strings: input.lines().map(|s| s.to_string()).collect(),
        }
    }
}

impl Solver<usize, usize> for Solver2015_08 {
    fn solve_first_part(&self) -> usize {
        self.strings
            .iter()
            .fold(0, |acc, s| acc + s.len() - count_memory_chars(s))
    }

    fn solve_second_part(&self) -> usize {
        self.strings
            .iter()
            .fold(0, |acc, s| acc + count_encoded_chars(s) - s.len())
    }
}

fn count_memory_chars(s: &str) -> usize {
    let mut chars = s[1..s.len() - 1].chars();
    let mut count = 0;
    while let Some(c) = chars.next() {
        count += 1;
        if c == '\\' {
            match chars.next() {
                Some('x') => {
                    chars.next();
                    chars.next();
                }
                Some('\\') | Some('"') => {}
                _ => count += 1,
            }
        }
    }
    count
}

fn count_encoded_chars(s: &str) -> usize {
    s.chars().fold(2, |acc, c| {
        acc + match c {
            '"' | '\\' => 2,
            _ => 1,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2015_08::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 12);
    }

    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2015_08::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 19);
    }
}
