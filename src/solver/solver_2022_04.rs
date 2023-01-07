use super::utils;

fn does_contain(bounds: &(i32, i32), other: &(i32, i32)) -> bool {
    bounds.0 <= other.0 && bounds.1 >= other.1
}

fn does_overlap(a: &(i32, i32), b: &(i32, i32)) -> bool {
    let (smaller, larger) = if a.0 < b.0 { (a, b) } else { (b, a) };
    smaller.1 >= larger.0
}

fn get_assignments() -> Vec<((i32, i32), (i32, i32))> {
    let input = utils::get_input("inputs/2022_04.txt");

    input
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
        .collect()
}

fn solve_first_part(assignments: &Vec<((i32, i32), (i32, i32))>) -> usize {
    assignments
        .iter()
        .filter(|(first_bound, second_bound)| {
            does_contain(first_bound, second_bound) || does_contain(second_bound, first_bound)
        })
        .count()
}

fn solve_second_part(assignments: &Vec<((i32, i32), (i32, i32))>) -> usize {
    assignments
        .iter()
        .filter(|(a, b)| does_overlap(a, b))
        .count()
}

pub fn solve() -> (usize, usize) {
    let assignments = get_assignments();
    (
        solve_first_part(&assignments),
        solve_second_part(&assignments),
    )
}

#[cfg(test)]
mod tests {
    static EXAMPLE: [((i32, i32), (i32, i32)); 6] = [
        ((2, 4), (6, 8)),
        ((2, 3), (4, 5)),
        ((5, 7), (7, 9)),
        ((2, 8), (3, 7)),
        ((6, 6), (4, 6)),
        ((2, 6), (4, 8)),
    ];
    #[test]
    fn solve_first_part() {
        assert_eq!(super::solve_first_part(&Vec::from(EXAMPLE)), 2);
    }
    #[test]
    fn solve_second_part() {
        assert_eq!(super::solve_second_part(&Vec::from(EXAMPLE)), 4);
    }
}
