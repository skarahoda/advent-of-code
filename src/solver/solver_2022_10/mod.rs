use super::Solver;
use regex::Regex;

#[derive(Clone)]
enum Operation {
    NoOp,
    Add(i32),
}

pub struct Solver2022_10 {
    operations: Vec<Operation>,
}

impl Default for Solver2022_10 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2022_10 {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"addx (-?\d+)").unwrap();

        let operations = input
            .split("\n")
            .map(|operation| match operation {
                "noop" => Operation::NoOp,
                other => Operation::Add(re.captures(other).unwrap()[1].parse().unwrap()),
            })
            .collect();
        Self { operations }
    }
}

fn get_pixel(cycle_count: i32, register: i32) -> String {
    let cycle_count = cycle_count % 40;
    let suffix = if cycle_count == 39 { "\n" } else { "" };
    if cycle_count >= register && cycle_count <= register + 2 {
        "#".to_owned() + suffix
    } else {
        ".".to_owned() + suffix
    }
}

impl Solver<i32, String> for Solver2022_10 {
    fn solve_first_part(&self) -> i32 {
        let mut cycle_count = 0;
        let mut register = 1;
        let mut result = 0;
        for operation in &self.operations {
            match operation {
                Operation::NoOp => {
                    cycle_count += 1;
                    match cycle_count % 40 {
                        20 => {
                            result += cycle_count * register;
                        }
                        _ => {}
                    }
                }
                Operation::Add(value) => {
                    cycle_count += 2;
                    match cycle_count % 40 {
                        20 => result += cycle_count * register,
                        21 => result += (cycle_count - 1) * register,
                        _ => {}
                    }
                    register += value;
                }
            }
        }
        result
    }

    fn solve_second_part(&self) -> String {
        let mut cycle_count = 0;
        let mut register = 0;
        let mut result = String::from("");
        for operation in &self.operations {
            match operation {
                Operation::NoOp => {
                    result += get_pixel(cycle_count, register).as_str();
                    cycle_count += 1;
                }
                Operation::Add(value) => {
                    result += get_pixel(cycle_count, register).as_str();
                    cycle_count += 1;

                    result += get_pixel(cycle_count, register).as_str();
                    cycle_count += 1;

                    register += value;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2022_10::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 13140);
    }

    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2022_10::from(EXAMPLE);
        assert_eq!(
            solver.solve_second_part(),
            "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
