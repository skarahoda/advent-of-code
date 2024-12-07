mod input;
use input::INPUT;

fn get_carried_foods() -> Vec<i32> {
    let carried_foods: Vec<Vec<i32>> = INPUT
        .split("\n\n")
        .map(|e| e.split("\n").map(|i| i.parse().unwrap()).collect())
        .collect();

    carried_foods.iter().map(|f| f.iter().sum()).collect()
}

fn solve_first_part(carried_foods: &Vec<i32>) -> i32 {
    match carried_foods.iter().max() {
        Some(value) => *value,
        None => 0,
    }
}
fn solve_second_part(carried_foods: &Vec<i32>) -> i32 {
    if carried_foods.len() <= 3 {
        carried_foods.iter().sum()
    } else {
        let mut result = vec![0; 3];
        for number in carried_foods {
            let number = *number;
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

pub fn solve() -> (i32, i32) {
    let carried_foods = get_carried_foods();
    (
        solve_first_part(&carried_foods),
        solve_second_part(&carried_foods),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_first_part() {
        assert_eq!(
            super::solve_first_part(&vec![6000, 11000, 24000, 10000]),
            24000
        );
    }
    #[test]
    fn solve_second_part() {
        assert_eq!(
            super::solve_second_part(&vec![6000, 11000, 24000, 10000]),
            45000
        );
    }
}
