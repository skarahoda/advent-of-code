use crate::solver::Solver;
use regex::Regex;

#[derive(Clone)]
pub struct Solver2022_05 {
    stacks: Vec<Vec<char>>,
    procedures: Vec<(usize, usize, usize)>,
}

impl Default for Solver2022_05 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}
impl From<&str> for Solver2022_05 {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split("\n\n").collect();
        let mut stack_drawing: Vec<&str> = parts[0].split("\n").collect();
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
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let procedures = re
            .captures_iter(parts[1])
            .map(|capture| {
                let count: usize = (&capture[1]).parse().unwrap();
                let from: usize = (&capture[2]).parse::<usize>().unwrap() - 1;
                let to: usize = (&capture[3]).parse::<usize>().unwrap() - 1;
                (count, from, to)
            })
            .collect();
        Self { stacks, procedures }
    }
}

impl Solver<String, String> for Solver2022_05 {
    fn solve_first_part(&self) -> String {
        let mut mutated = self.clone();
        for (count, from, to) in mutated.procedures.iter() {
            let new_length = mutated.stacks[*from].len() - *count;
            let mut popped_items = mutated.stacks[*from].split_off(new_length);
            popped_items.reverse();
            mutated.stacks[*to].extend(popped_items);
        }
        mutated
            .stacks
            .iter_mut()
            .map(|stack| stack.pop().unwrap())
            .collect()
    }

    fn solve_second_part(&self) -> String {
        let mut mutated = self.clone();
        for (count, from, to) in mutated.procedures.iter() {
            let new_length = mutated.stacks[*from].len() - *count;
            let popped_items = mutated.stacks[*from].split_off(new_length);
            mutated.stacks[*to].extend(popped_items);
        }

        mutated
            .stacks
            .iter_mut()
            .map(|stack| stack.pop().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn solve_first_part() {
        let solver = Solver2022_05::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), "CMZ");
    }
    #[test]
    fn solve_second_part() {
        let solver = Solver2022_05::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), "MCD");
    }
}
