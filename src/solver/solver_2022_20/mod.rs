use super::Solver;

pub struct Solver2022_20 {
    numbers: Vec<isize>,
}

impl Default for Solver2022_20 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2022_20 {
    fn from(input: &str) -> Self {
        let numbers = input
            .split("\n")
            .map(|number| number.parse().unwrap())
            .collect();
        Self { numbers }
    }
}

impl Solver2022_20 {
    fn decrypt(&self, key: isize, rounds: usize) -> isize {
        let mut numbers: Vec<(usize, isize)> = self
            .numbers
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, num)| (i, num * key))
            .collect();
        for _ in 0..rounds {
            for original_index in 0..numbers.len() {
                let index = numbers.iter().position(|x| x.0 == original_index).unwrap();
                let value = numbers[index].1;
                let new_index = index as isize + value;

                let new_index = new_index.rem_euclid(numbers.len() as isize - 1);
                let new_index = if new_index == 0 {
                    numbers.len() as isize - 1
                } else {
                    new_index
                };

                let tmp = numbers.remove(index);
                numbers.insert(new_index as usize, tmp);
            }
        }

        let zero = numbers.iter().position(|x| x.1 == 0).unwrap();
        let x1 = numbers[(zero + 1_000) % numbers.len()].1;
        let x2 = numbers[(zero + 2_000) % numbers.len()].1;
        let x3 = numbers[(zero + 3_000) % numbers.len()].1;
        x1 + x2 + x3
    }
}

impl Solver<isize, isize> for Solver2022_20 {
    fn solve_first_part(&self) -> isize {
        self.decrypt(1, 1)
    }

    fn solve_second_part(&self) -> isize {
        self.decrypt(811589153, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
        1\n\
        2\n\
        -3\n\
        3\n\
        -2\n\
        0\n\
        4\
    ";

    #[test]
    fn should_solve_first_part() {
        let solver = Solver2022_20::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 3);
    }

    #[test]
    fn should_solve_second_part() {
        let solver = Solver2022_20::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 1623178306);
    }
}
