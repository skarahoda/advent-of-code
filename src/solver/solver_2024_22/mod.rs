use super::Solver;
use std::collections::HashMap;

pub struct Solver2024_22 {
    secret_numbers: Vec<usize>,
}

impl Default for Solver2024_22 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2024_22 {
    fn from(input: &str) -> Self {
        Self {
            secret_numbers: input.lines().map(|line| line.parse().unwrap()).collect(),
        }
    }
}

fn get_next_secret_number(secret_number: usize) -> usize {
    let mut secret_number = ((secret_number * 64) ^ secret_number) % 16777216;
    secret_number = ((secret_number / 32) ^ secret_number) % 16777216;
    secret_number = ((secret_number * 2048) ^ secret_number) % 16777216;
    secret_number
}

fn get_nth_secret_number(secret_number: usize, n: usize) -> usize {
    let mut secret_number = secret_number;
    for _ in 0..n {
        secret_number = get_next_secret_number(secret_number);
    }
    secret_number
}

fn get_prices_and_diffs(secret_number: &usize, n: usize) -> Vec<(usize, isize)> {
    let mut secret_number = *secret_number;
    (0..n)
        .map(|_| {
            let next = get_next_secret_number(secret_number);
            let price = next % 10;
            let diff = (price as isize) - (secret_number % 10) as isize;
            secret_number = next;
            (price, diff)
        })
        .collect()
}

fn get_price_map_for_sequences(
    prices_and_diffs: &Vec<Vec<(usize, isize)>>,
    sequence_length: usize,
) -> HashMap<Vec<isize>, usize> {
    let mut result = HashMap::new();
    for buyer in prices_and_diffs {
        let mut added: HashMap<Vec<isize>, usize> = HashMap::new();
        for i in sequence_length - 1..buyer.len() {
            let sequence: Vec<isize> = buyer[i + 1 - sequence_length..=i]
                .iter()
                .map(|&(_, diff)| diff)
                .collect();
            if added.contains_key(&sequence) {
                continue;
            }
            added.insert(sequence.iter().cloned().collect(), buyer[i].0);
            *result.entry(sequence).or_default() += buyer[i].0;
        }
    }
    result
}

impl Solver<usize, usize> for Solver2024_22 {
    fn solve_first_part(&self) -> usize {
        self.secret_numbers
            .iter()
            .map(|secret_number| get_nth_secret_number(*secret_number, 2000))
            .sum()
    }

    fn solve_second_part(&self) -> usize {
        let prices_and_diffs: Vec<Vec<(usize, isize)>> = self
            .secret_numbers
            .iter()
            .map(|secret_number| get_prices_and_diffs(secret_number, 2000))
            .collect();
        let price_map = get_price_map_for_sequences(&prices_and_diffs, 4);
        price_map.iter().map(|(_, &price)| price).max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_next_secret_number() {
        assert_eq!(get_next_secret_number(123), 15887950);
        assert_eq!(get_next_secret_number(15887950), 16495136);
        assert_eq!(get_next_secret_number(16495136), 527345);
        assert_eq!(get_next_secret_number(527345), 704524);
        assert_eq!(get_next_secret_number(704524), 1553684);
        assert_eq!(get_next_secret_number(1553684), 12683156);
        assert_eq!(get_next_secret_number(12683156), 11100544);
        assert_eq!(get_next_secret_number(11100544), 12249484);
        assert_eq!(get_next_secret_number(12249484), 7753432);
        assert_eq!(get_next_secret_number(7753432), 5908254);
    }

    #[test]
    fn test_get_nth_secret_number() {
        assert_eq!(get_nth_secret_number(123, 0), 123);
        assert_eq!(get_nth_secret_number(123, 1), 15887950);
        assert_eq!(get_nth_secret_number(123, 2), 16495136);
        assert_eq!(get_nth_secret_number(123, 3), 527345);
        assert_eq!(get_nth_secret_number(123, 4), 704524);
        assert_eq!(get_nth_secret_number(123, 5), 1553684);
        assert_eq!(get_nth_secret_number(123, 6), 12683156);
        assert_eq!(get_nth_secret_number(123, 7), 11100544);
        assert_eq!(get_nth_secret_number(123, 8), 12249484);
        assert_eq!(get_nth_secret_number(123, 9), 7753432);
        assert_eq!(get_nth_secret_number(123, 10), 5908254);
    }

    #[test]
    fn test_get_prices_and_diffs() {
        assert_eq!(
            get_prices_and_diffs(&123, 10),
            vec![
                (0, -3),
                (6, 6),
                (5, -1),
                (4, -1),
                (4, 0),
                (6, 2),
                (4, -2),
                (4, 0),
                (2, -2),
            ]
        );
    }

    #[test]
    fn test_solve_first_part() {
        let solver = Solver2024_22::from(include_str!("example.txt"));
        assert_eq!(solver.solve_first_part(), 37327623);
    }

    #[test]
    fn test_solve_second_part() {
        let solver = Solver2024_22::from(include_str!("example2.txt"));
        assert_eq!(solver.solve_second_part(), 23);
    }
    #[test]
    fn test_solve_second_part_example3() {
        let solver = Solver2024_22::from(include_str!("example3.txt"));
        assert_eq!(solver.solve_second_part(), 27);
    }
}
