use super::Solver;

pub struct Solver2015_12<'a> {
    input: &'a str,
}

impl Default for Solver2015_12<'_> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2015_12<'a> {
    fn from(input: &'a str) -> Self {
        Self { input }
    }
}

impl Solver2015_12<'_> {}

impl Solver<isize, usize> for Solver2015_12<'_> {
    fn solve_first_part(&self) -> isize {
        let mut result = 0isize;
        let mut current = 0;
        let mut is_negative = false;
        for c in self.input.chars() {
            if c == '-' {
                is_negative = true;
            } else if let Some(digit) = c.to_digit(10) {
                current = current * 10 + digit;
            } else {
                result += if is_negative {
                    (current as isize) * -1
                } else {
                    current as isize
                };
                current = 0;
                is_negative = false;
            }
        }
        result
            + if is_negative {
                (current as isize) * -1
            } else {
                current as isize
            }
    }

    fn solve_second_part(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2015_12::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 605);
    }

    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2015_12::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 982);
    }
}
