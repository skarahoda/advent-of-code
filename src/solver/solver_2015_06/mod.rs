use super::Solver;
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

pub struct Solver2015_06 {
    instructions: Vec<Instruction>,
}

impl Default for Solver2015_06 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}
impl<'a> From<&'a str> for Solver2015_06 {
    fn from(input: &'a str) -> Self {
        let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();

        Self {
            instructions: re
                .captures_iter(input)
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
                .collect(),
        }
    }
}

impl Solver<usize, usize> for Solver2015_06 {
    fn solve_first_part(&self) -> usize {
        let mut lights = [[false; 1000]; 1000];

        for instruction in self.instructions.iter() {
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
    fn solve_second_part(&self) -> usize {
        let mut lights = [[0usize; 1000]; 1000];

        for instruction in self.instructions.iter() {
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
}
