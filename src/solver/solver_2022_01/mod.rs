use super::Solver;

pub struct Solver2022_01 {
    carried_foods: Vec<i32>,
}

impl Default for Solver2022_01 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}
impl From<&str> for Solver2022_01 {
    fn from(input: &str) -> Self {
        Self {
            carried_foods: input
                .split("\n\n")
                .map(|e| e.lines().map(|i| i.parse::<i32>().unwrap()).sum())
                .collect(),
        }
    }
}

impl Solver<i32, i32> for Solver2022_01 {
    fn solve_first_part(&self) -> i32 {
        self.carried_foods.iter().max().unwrap().clone()
    }

    fn solve_second_part(&self) -> i32 {
        let mut result = vec![0; 3];
        for &number in &self.carried_foods {
            if number > result[0] {
                result[2] = result[1];
                result[1] = result[0];
                result[0] = number;
            } else if number > result[1] {
                result[2] = result[1];
                result[1] = number;
            } else if number > result[2] {
                result[2] = number;
            }
        }
        result.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn solve_first_part() {
        let solver = Solver2022_01::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 24000);
    }
    #[test]
    fn solve_second_part() {
        let solver = Solver2022_01::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 45000);
    }
}
