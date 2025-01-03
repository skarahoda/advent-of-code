use super::Solver;

pub struct Solver2015_17 {
    total: usize,
    containers: Vec<usize>,
}

impl Default for Solver2015_17 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2015_17 {
    fn from(input: &str) -> Self {
        let parts = input.split("\n\n").collect::<Vec<_>>();
        Self {
            total: parts[0].parse().unwrap(),
            containers: parts[1].lines().map(|line| line.parse().unwrap()).collect(),
        }
    }
}

fn get_combinations(total: usize, containers: &[usize]) -> Vec<Vec<usize>> {
    if containers.len() == 0 {
        return Vec::new();
    }
    let &first_container = containers.first().unwrap();
    let mut result = get_combinations(total, &containers[1..]);
    if first_container > total {
        return result;
    }
    if first_container == total {
        result.push(vec![first_container]);
        return result;
    }
    result.extend(
        get_combinations(total - first_container, &containers[1..])
            .into_iter()
            .map(|mut combination| {
                combination.push(first_container);
                combination
            }),
    );
    result
}
impl Solver<usize, usize> for Solver2015_17 {
    fn solve_first_part(&self) -> usize {
        get_combinations(self.total, &self.containers).len()
    }

    fn solve_second_part(&self) -> usize {
        let result = get_combinations(self.total, &self.containers);
        let min_length = result
            .iter()
            .map(|combination| combination.len())
            .min()
            .unwrap();
        result
            .iter()
            .filter(|combination| combination.len() == min_length)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2015_17::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 4);
    }

    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2015_17::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 0);
    }
}
