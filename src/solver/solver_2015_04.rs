use md5;


fn solve_first_part(input: &str) -> u32 {
    let mut number: u32 = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, number));
        if format!("{:x}", hash).starts_with("00000") {
            return number;
        }
        number += 1;
    }
}

fn solve_second_part(input: &str) -> u32 {
    let mut number: u32 = 0;
    loop {
        let hash = md5::compute(format!("{}{}", input, number));
        if format!("{:x}", hash).starts_with("000000") {
            return number;
        }
        number += 1;
    }
}

pub fn solve() -> (u32, u32) {
    (
        solve_first_part("bgvyzdsv"),
        solve_second_part("bgvyzdsv")
    )
}

#[cfg(test)]
mod first_part {
    #[test]
    fn solve_first_example() {
        assert_eq!(super::solve_first_part("abcdef"), 609043);
    }
    #[test]
    fn solve_second_example() {
        assert_eq!(super::solve_first_part("pqrstuv"), 1048970);
    }
}
