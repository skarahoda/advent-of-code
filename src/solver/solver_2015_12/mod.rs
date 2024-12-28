use super::Solver;

pub struct Solver2015_12 {
    value: serde_json::Value,
}

impl Default for Solver2015_12 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2015_12 {
    fn from(input: &str) -> Self {
        Self {
            value: serde_json::from_str(input).unwrap(),
        }
    }
}

fn get_total(value: &serde_json::Value, ignore: Option<&str>) -> i64 {
    match value {
        serde_json::Value::Number(number) => number.as_i64().unwrap(),
        serde_json::Value::Array(array) => array.iter().map(|value| get_total(value, ignore)).sum(),
        serde_json::Value::Object(object) => {
            if ignore.is_some_and(|ignore| {
                object
                    .values()
                    .any(|v| v.as_str().is_some_and(|s| s == ignore))
            }) {
                0
            } else {
                object
                    .iter()
                    .map(|(_, value)| get_total(value, ignore))
                    .sum()
            }
        }
        _ => 0,
    }
}

impl Solver<i64, i64> for Solver2015_12 {
    fn solve_first_part(&self) -> i64 {
        get_total(&self.value, None)
    }

    fn solve_second_part(&self) -> i64 {
        get_total(&self.value, Some("red"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2015_12::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 6);
    }

    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2015_12::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 4);
    }
}
