use super::Solver;

mod input;
use input::INPUT;

#[derive(Clone)]
pub struct Solver2024_11 {
    rocks: Vec<usize>,
}

impl<'a> Default for Solver2024_11 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

impl<'a> From<&'a str> for Solver2024_11 {
    fn from(input: &'a str) -> Self {
        Self {
            rocks: input
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }
}

impl Solver2024_11 {
    fn blink(&mut self) {
        let len = self.rocks.len();
        for i in 0..len {
            let rock = self.rocks.get_mut(i).unwrap();
            let num_of_digits = if *rock == 0 { 1 } else { rock.ilog10() + 1 };

            if *rock == 0 {
                *rock = 1;
            } else if num_of_digits % 2 == 0 {
                let divisor = 10_usize.pow(num_of_digits / 2);
                let left = *rock / divisor;
                let right = *rock % divisor;
                *rock = right;
                self.rocks.push(left);
            } else {
                *rock *= 2024;
            }
        }
    }
}

impl Solver<usize, usize> for Solver2024_11 {
    fn solve_first_part(&self) -> usize {
        let mut muted = self.clone();
        for _ in 0..25 {
            muted.blink();
        }
        muted.rocks.len()
    }

    fn solve_second_part(&self) -> usize {
        panic!("second part is not working");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "125 17";

    #[test]
    fn should_solve_first_part() {
        let solver = Solver2024_11::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 55312);
    }
}
