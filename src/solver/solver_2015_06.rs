use super::utils;
use regex::{Captures, Regex};

enum Command {
    On,
    Off,
    Toggle,
}

struct Instruction {
    start: (usize, usize),
    end: (usize, usize),
    command: Command,
}

fn get_instructions() -> Vec<Instruction> {
    let input = utils::get_input("inputs/2015_06.txt");
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let input: &str = &input[..];
    re.captures_iter(input)
        .map(|captures: Captures| Instruction {
            start: (
                (&captures)[2].parse().unwrap(),
                (&captures)[3].parse().unwrap(),
            ),
            end: (
                (&captures)[4].parse().unwrap(),
                (&captures)[5].parse().unwrap(),
            ),
            command: match &(&captures)[1] {
                "turn on" => Command::On,
                "turn off" => Command::Off,
                "toggle" => Command::Toggle,
                other => panic!("Illegal argument: {}", other),
            },
        })
        .collect()
}

fn solve_first_part(instructions: &Vec<Instruction>) -> usize {
    let mut lights = [[false; 1000]; 1000];

    for instruction in instructions {
        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                match instruction.command {
                    Command::On => lights[x][y] = true,
                    Command::Off => lights[x][y] = false,
                    Command::Toggle => lights[x][y] = !lights[x][y],
                }
            }
        }
    }

    lights.iter().fold(0, |acc: usize, row: &[bool; 1000]| {
        row.iter().fold(
            acc,
            |acc: usize, light: &bool| if *light { acc + 1 } else { acc },
        )
    })
}
fn solve_second_part(instructions: &Vec<Instruction>) -> usize {
    let mut lights = [[0usize; 1000]; 1000];

    for instruction in instructions {
        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                match instruction.command {
                    Command::On => lights[x][y] = lights[x][y] + 1,
                    Command::Off => lights[x][y] = lights[x][y].checked_sub(1).unwrap_or(0),
                    Command::Toggle => lights[x][y] = lights[x][y] + 2,
                }
            }
        }
    }

    lights.iter().fold(0, |acc: usize, row: &[usize; 1000]| {
        row.iter()
            .fold(acc, |acc: usize, light: &usize| acc + light)
    })
}

pub fn solve() -> (usize, usize) {
    let instructions = get_instructions();
    (
        solve_first_part(&instructions),
        solve_second_part(&instructions),
    )
}
