mod input;
use input::INPUT;
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
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    re.captures_iter(INPUT)
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

    lights
        .iter()
        .map(|row: &[bool; 1000]| row.iter().filter(|light| **light).count())
        .sum()
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

    lights
        .iter()
        .map(|row: &[usize; 1000]| row.iter().sum::<usize>())
        .sum()
}

pub fn solve() -> (usize, usize) {
    let instructions = get_instructions();
    (
        solve_first_part(&instructions),
        solve_second_part(&instructions),
    )
}
