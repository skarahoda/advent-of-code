use super::Solver;
mod input;
use input::INPUT;
use regex::Regex;

pub struct Solver202403<'a> {
    input: &'a str,
}

impl Default for Solver202403<'static> {
    fn default() -> Self {
        Self { input: INPUT }
    }
}

impl<'a> From<&'a str> for Solver202403<'a> {
    fn from(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'a> Solver<i32, i32> for Solver202403<'a> {
    fn solve_first_part(&self) -> i32 {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        re.captures_iter(self.input)
            .map(|captures| {
                let a: i32 = captures[1].parse().unwrap();
                let b: i32 = captures[2].parse().unwrap();
                a * b
            })
            .sum()
    }

    fn solve_second_part(&self) -> i32 {
        let input = self.input.replace("\n", "");
        let input = format!("{input}do()");
        let re = Regex::new(r"(don't\(\)).*?(do\(\))").unwrap();
        let input = re.replace_all(&input, "").to_string();
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        re.captures_iter(&input)
            .map(|captures| {
                let a: i32 = captures[1].parse().unwrap();
                let b: i32 = captures[2].parse().unwrap();
                a * b
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_first_part() {
        let solver = Solver202403::from(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(solver.solve_first_part(), 161);
    }

    #[test]
    fn solve_second_part() {
        let solver = Solver202403::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(solver.solve_second_part(), 48);
    }
}
