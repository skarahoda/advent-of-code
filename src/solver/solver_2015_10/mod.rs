use super::Solver;

pub struct Solver2015_10 {
    input: Vec<u8>,
}

impl Default for Solver2015_10 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2015_10 {
    fn from(input: &str) -> Self {
        Self {
            input: input
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        }
    }
}

fn look_and_say(input: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    result.reserve(input.len() * 2);
    let mut iter = input.iter();
    let mut current = *iter.next().unwrap();
    let mut count: u8 = 1;
    for &i in iter {
        if i == current {
            count += 1;
        } else {
            let mut v = count;
            while v > 0 {
                let n = (v % 10) as u8;
                v /= 10;
                result.push(n);
            }
            result.push(current);
            current = i;
            count = 1;
        }
    }
    let mut v = count;
    while v > 0 {
        let n = (v % 10) as u8;
        v /= 10;
        result.push(n);
    }
    result.push(current);
    result
}

impl Solver2015_10 {
    fn apply_look_and_say_n_times(&self, n: usize) -> usize {
        let mut result = self.input.clone();
        for _ in 0..n {
            result = look_and_say(&result);
        }
        result.len()
    }
}

impl Solver<usize, usize> for Solver2015_10 {
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
        assert_eq!(look_and_say(&vec![1]), vec![1, 1]);
    }

    #[test]
    fn should_look_and_say_11() {
        assert_eq!(look_and_say(&vec![1, 1]), vec![2, 1]);
    }

    #[test]
    fn should_look_and_say_21() {
        assert_eq!(look_and_say(&vec![2, 1]), vec![1, 2, 1, 1]);
    }

    #[test]
    fn should_look_and_say_1211() {
        assert_eq!(look_and_say(&vec![1, 2, 1, 1]), vec![1, 1, 1, 2, 2, 1]);
    }

    #[test]
    fn should_look_and_say_111221() {
        assert_eq!(
            look_and_say(&vec![1, 1, 1, 2, 2, 1]),
            vec![3, 1, 2, 2, 1, 1]
        );
    }
}
