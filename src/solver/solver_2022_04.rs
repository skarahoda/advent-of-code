use super::utils;

fn does_contain(bounds: &(i32,i32), other: &(i32,i32)) -> bool {
    bounds.0 <= other.0 && bounds.1 >= other.1
}

fn does_overlap(a: &(i32,i32), b: &(i32,i32)) -> bool {
    let (smaller, larger) = if a.0 < b.0 { (a,b) } else { (b,a) };
    smaller.1 >= larger.0
}

fn get_assignments() -> Vec<((i32, i32), (i32, i32))> {
    let input = utils::get_input("inputs/2022_04.txt");

    input.split("\n").map(
        |row| {
            let assignments: Vec<(i32, i32)> = row.split(",")
                .map(|assignment| {
                    let bounds: Vec<i32> = assignment.split("-")
                        .map(|bound| bound.parse().unwrap())
                        .collect();
                    (bounds[0], bounds[1])
                })
                .collect();
            (assignments[0], assignments[1])
        }
    ).collect()
}

pub fn solve_first_part() -> usize {
    get_assignments().iter()
        .filter(|(first_bound, second_bound)| does_contain(first_bound, second_bound) || does_contain(second_bound, first_bound))
        .count()
}

pub fn solve_second_part() -> usize {
    get_assignments().iter()
        .filter(|(a, b)| does_overlap(a,b))
        .count()
}
