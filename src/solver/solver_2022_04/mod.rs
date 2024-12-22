use super::Solver;

fn does_contain(bounds: &(i32, i32), other: &(i32, i32)) -> bool {
    bounds.0 <= other.0 && bounds.1 >= other.1
}

fn does_overlap(a: &(i32, i32), b: &(i32, i32)) -> bool {
    let (smaller, larger) = if a.0 < b.0 { (a, b) } else { (b, a) };
    smaller.1 >= larger.0
}

pub struct Solver2022_04 {
    assignments: Vec<((i32, i32), (i32, i32))>,
}

impl Default for Solver2022_04 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}
impl From<&str> for Solver2022_04 {
    fn from(input: &str) -> Self {
        Self {
            assignments: input
                .split("\n")
                .map(|row| {
                    let assignments: Vec<(i32, i32)> = row
                        .split(",")
                        .map(|assignment| {
                            let bounds: Vec<i32> = assignment
                                .split("-")
                                .map(|bound| bound.parse().unwrap())
                                .collect();
                            (bounds[0], bounds[1])
                        })
                        .collect();
                    (assignments[0], assignments[1])
                })
                .collect(),
        }
    }
}

impl Solver<usize, usize> for Solver2022_04 {
    fn solve_first_part(&self) -> usize {
        self.assignments
            .iter()
            .filter(|(first_bound, second_bound)| {
                does_contain(first_bound, second_bound) || does_contain(second_bound, first_bound)
            })
            .count()
    }

    fn solve_second_part(&self) -> usize {
        self.assignments
            .iter()
            .filter(|(a, b)| does_overlap(a, b))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn solve_first_part() {
        let solver = Solver2022_04::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 2);
    }
    #[test]
    fn solve_second_part() {
        let solver = Solver2022_04::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 4);
    }
}
