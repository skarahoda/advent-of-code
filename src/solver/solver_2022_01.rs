use std::fs;

fn get_carried_foods() -> Vec<i32> {
    let mut content = fs::read_to_string("inputs/2022_01.txt")
        .expect("Should have been able to read the file");
    if content.chars().last().unwrap() == "\n".chars().next().unwrap() {
        content.pop();
    }
    let carried_foods: Vec<Vec<i32>> = content.split("\n\n").map(
        |e| e.split("\n")
            .map(|i| i.parse().unwrap())
            .collect()
    ).collect();

    carried_foods.iter().map(
        |f| f.iter().sum()
    ).collect()
}

fn sum_top_three(numbers: Vec<i32>) -> i32 {
    if numbers.len() <= 3 {
        numbers.iter().sum()
    } else {
        let mut result = vec![0; 3];
        for number in numbers {
            if  number > result[0] {
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

pub fn solve_first_part() -> i32 {
    let carried_foods = get_carried_foods();
    match carried_foods.iter().max() {
        Some(value) => *value,
        None => 0
    }
}
pub fn solve_second_part() -> i32 {

    let carried_foods = get_carried_foods();
    sum_top_three(carried_foods)
}
