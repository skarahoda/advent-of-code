use regex::{Captures, Regex};

fn get_setup() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let parts: Vec<&str> = include_str!("input.txt").split("\n\n").collect();
    (get_stacks(parts[0]), get_procedures(parts[1]))
}

fn get_stacks(stack_drawing: &str) -> Vec<Vec<char>> {
    let mut stack_drawing: Vec<&str> = stack_drawing.split("\n").collect();
    let number_of_stacks: usize = stack_drawing
        .pop()
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>()
        .pop()
        .unwrap()
        .parse()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); number_of_stacks];
    for line in stack_drawing.iter().rev() {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..(chars.len() + 1) / 4 {
            let item = chars[i * 4 + 1];
            if item != ' ' {
                stacks[i].push(chars[i * 4 + 1]);
            }
        }
    }
    stacks
}

fn get_procedures(lines: &str) -> Vec<(usize, usize, usize)> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    re.captures_iter(lines)
        .map(|capture: Captures| {
            let count: usize = (&capture[1]).parse().unwrap();
            let from: usize = (&capture[2]).parse::<usize>().unwrap() - 1;
            let to: usize = (&capture[3]).parse::<usize>().unwrap() - 1;
            (count, from, to)
        })
        .collect()
}

fn solve_first_part(mut stacks: Vec<Vec<char>>, procedures: &Vec<(usize, usize, usize)>) -> String {
    for (count, from, to) in procedures.iter() {
        let new_length = stacks[*from].len() - *count;
        let mut popped_items = stacks[*from].split_off(new_length);
        popped_items.reverse();
        stacks[*to].extend(popped_items);
    }
    stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect()
}

fn solve_second_part(
    mut stacks: Vec<Vec<char>>,
    procedures: &Vec<(usize, usize, usize)>,
) -> String {
    for (count, from, to) in procedures.iter() {
        let new_length = stacks[*from].len() - *count;
        let popped_items = stacks[*from].split_off(new_length);
        stacks[*to].extend(popped_items);
    }

    stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect()
}

pub fn solve() -> (String, String) {
    let (stacks, procedures) = get_setup();
    (
        solve_first_part(stacks.clone(), &procedures),
        solve_second_part(stacks.clone(), &procedures),
    )
}

#[cfg(test)]
mod tests {
    static EXAMPLE_PROCEDURES: [(usize, usize, usize); 4] =
        [(1, 1, 0), (3, 0, 2), (2, 1, 0), (1, 0, 1)];
    #[test]
    fn solve_first_part() {
        let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(
            super::solve_first_part(stacks, &Vec::from(EXAMPLE_PROCEDURES)),
            "CMZ"
        );
    }
    #[test]
    fn solve_second_part() {
        let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(
            super::solve_second_part(stacks, &Vec::from(EXAMPLE_PROCEDURES)),
            "MCD"
        );
    }
}
