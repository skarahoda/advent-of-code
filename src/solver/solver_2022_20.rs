use super::utils;

fn get_numbers(input: &str) -> Vec<isize> {
    input.split("\n").map(|number| number.parse().unwrap()).collect()
}

fn decrypt(numbers: &Vec<isize>, key: isize, rounds: usize) -> isize {
    let mut numbers: Vec<(usize, isize)> = numbers.iter().enumerate().map(|(i, &number )| (i, number * key)).collect();
    for _ in 0..rounds {
        for original_index in 0..numbers.len() {
            let index = numbers.iter().position(|x| x.0 == original_index).unwrap();
            let value = numbers[index].1;
            let new_index = index as isize + value;

            let new_index = new_index.rem_euclid(numbers.len() as isize - 1);
            let new_index= if new_index == 0 { numbers.len() as isize - 1 } else { new_index };

            let tmp = numbers.remove(index);
            numbers.insert(new_index as usize, tmp);

            if value == -4241 {
                dbg!(index, new_index);
            }
        }
    }

    let zero = numbers.iter().position(|x| x.1 == 0).unwrap();
    let x1 = numbers[(zero + 1_000) % numbers.len()].1;
    let x2 = numbers[(zero + 2_000) % numbers.len()].1;
    let x3 = numbers[(zero + 3_000) % numbers.len()].1;
    x1 + x2 + x3
}


fn solve_first_part(numbers: &Vec<isize>) -> isize {
    decrypt(numbers, 1, 1)
}

fn solve_second_part(numbers: &Vec<isize>) -> isize {
    decrypt(numbers, 811589153, 10)
}

pub fn solve() -> (isize, isize) {
    let numbers = get_numbers(&utils::get_input("inputs/2022_20.txt"));
    (
        solve_first_part(&numbers),
        solve_second_part(&numbers),
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
        1\n\
        2\n\
        -3\n\
        3\n\
        -2\n\
        0\n\
        4\
    ";

    #[test]
    fn should_get_numbers() {
        assert_eq!(
            get_numbers(EXAMPLE),
            vec![1, 2, -3, 3, -2, 0, 4]
        );
    }

    #[test]
    fn should_solve_first_part() {
        assert_eq!(
            solve_first_part(&get_numbers(EXAMPLE)),
            3
        );
    }

    #[test]
    fn should_solve_second_part() {
        assert_eq!(
            solve_second_part(&get_numbers(EXAMPLE)),
            1623178306
        );
    }
}

