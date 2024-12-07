mod input;
use input::INPUT;
use regex::Regex;

fn solve_first_part(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|captures| {
            let a: i32 = captures[1].parse().unwrap();
            let b: i32 = captures[2].parse().unwrap();
            a * b
        })
        .sum()
}
fn solve_second_part(input: &str) -> i32 {
    let input = input.replace("\n", "");
    let input = format!("{input}do()");
    let re = Regex::new(r"(don't\(\)).*?(do\(\))").unwrap();
    let input = re.replace_all(&input, "").to_string();
    solve_first_part(&input)
}

pub fn solve() -> (i32, i32) {
    (
        solve_first_part(INPUT),
        solve_second_part(INPUT),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_first_part() {
        assert_eq!(
            super::solve_first_part("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn solve_second_part() {
        assert_eq!(
            super::solve_second_part("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
