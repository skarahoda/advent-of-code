use std::fs;

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

fn main() {
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

    let grouped_carried_foods = carried_foods.iter().map(
        |f| f.iter().sum()
    ).collect();
    dbg!(sum_top_three(grouped_carried_foods));
}
