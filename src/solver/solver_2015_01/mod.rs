use super::Solver;

pub struct Solver2015_01<'a> {
    input: &'a str,
}

impl Default for Solver2015_01<'_> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2015_01<'a> {
    fn from(input: &'a str) -> Self {
        Self { input }
    }
}

impl Solver<i32, usize> for Solver2015_01<'_> {
    fn solve_first_part(&self) -> i32 {
        self.input
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            })
            .sum()
    }

    fn solve_second_part(&self) -> usize {
        let mut score = 0;

        for (i, char) in self.input.chars().enumerate() {
            match char {
                '(' => score += 1,
                ')' => score -= 1,
                _ => unreachable!(),
            }
            if score < 0 {
                return i + 1;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod first_part {
    use super::*;
    #[test]
    fn same_number_of_closing_and_opening_parenthesis() {
        let solver = Solver2015_01::from("()()");
        assert_eq!(solver.solve_first_part(), 0);
    }
    #[test]
    fn has_more_closing_parenthesis() {
        let solver = Solver2015_01::from("()()))");
        assert_eq!(solver.solve_first_part(), -2);
    }
    #[test]
    fn has_more_opening_parenthesis() {
        let solver = Solver2015_01::from("()()((");
        assert_eq!(solver.solve_first_part(), 2);
    }
}

#[cfg(test)]
mod second_part {
    use super::*;
    #[test]
    fn enter_basement_with_first_character() {
        let solver = Solver2015_01::from(")");
        assert_eq!(solver.solve_second_part(), 1);
    }
    #[test]
    fn enter_basement_with_last_character() {
        let solver = Solver2015_01::from("(()))");
        assert_eq!(solver.solve_second_part(), 5);
    }
    #[test]
    fn enter_basement_in_the_middle() {
        let solver = Solver2015_01::from("()))()())()())");
        assert_eq!(solver.solve_second_part(), 3);
    }
}
