use regex::Regex;
use super::utils;

#[derive(Clone)]
enum Operation {
    NoOp,
    Add(i32),
}

fn get_operations() -> Vec<Operation> {
    let input = utils::get_input("inputs/2022_10.txt");
    let re = Regex::new(r"addx (-?\d+)").unwrap();

    input.split("\n").map(|operation| {
        match operation {
            "noop" => Operation::NoOp,
            other => {
                Operation::Add(re.captures(other).unwrap()[1].parse().unwrap())
            }
        }
    }).collect()
}

fn solve_first_part(operations: &Vec<Operation>) -> i32 {
    let mut cycle_count = 0;
    let mut register = 1;
    let mut result = 0;
    for operation in operations {
        match operation {
            Operation::NoOp => {
                cycle_count += 1;
                match cycle_count % 40 {
                    20 => {
                        result += cycle_count * register;
                    },
                    _ => {}
                }
            },
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

fn get_pixel(cycle_count: i32, register: i32) -> String {
    let cycle_count = cycle_count % 40;
    let suffix = if cycle_count == 39 { "\n" } else { ""};
    if cycle_count >= register && cycle_count <= register + 2 {
        "#".to_owned() + suffix
    } else {
         ".".to_owned() + suffix
    }
}

fn solve_second_part(operations: &Vec<Operation>) -> String {
    let mut cycle_count = 0;
    let mut register = 0;
    let mut result = String::from("");
    for operation in operations {
        match operation {
            Operation::NoOp => {
                result += get_pixel(cycle_count, register).as_str();
                cycle_count +=1;
            },
            Operation::Add(value) => {
                result += get_pixel(cycle_count, register).as_str();
                cycle_count +=1 ;

                result += get_pixel(cycle_count, register).as_str();
                cycle_count += 1;

                register += value;
            }
        }
    }
    result
}

pub fn solve() -> (i32, String) {
    let operations = get_operations();
    (
        solve_first_part(&operations),
        solve_second_part(&operations),
    )
}


#[cfg(test)]
mod tests {
    use super::Operation;

    static EXAMPLE: [Operation;146] =  [
        Operation::Add(15),
        Operation::Add(-11),
        Operation::Add(6),
        Operation::Add(-3),
        Operation::Add(5),
        Operation::Add(-1),
        Operation::Add(-8),
        Operation::Add(13),
        Operation::Add(4),
        Operation::NoOp,
        Operation::Add(-1),
        Operation::Add(5),
        Operation::Add(-1),
        Operation::Add(5),
        Operation::Add(-1),
        Operation::Add(5),
        Operation::Add(-1),
        Operation::Add(5),
        Operation::Add(-1),
        Operation::Add(-35),
        Operation::Add(1),
        Operation::Add(24),
        Operation::Add(-19),
        Operation::Add(1),
        Operation::Add(16),
        Operation::Add(-11),
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(21),
        Operation::Add(-15),
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(-3),
        Operation::Add(9),
        Operation::Add(1),
        Operation::Add(-3),
        Operation::Add(8),
        Operation::Add(1),
        Operation::Add(5),
        Operation::NoOp,
        Operation::NoOp,
        Operation::NoOp,
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(-36),
        Operation::NoOp,
        Operation::Add(1),
        Operation::Add(7),
        Operation::NoOp,
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(2),
        Operation::Add(6),
        Operation::NoOp,
        Operation::NoOp,
        Operation::NoOp,
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(1),
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(7),
        Operation::Add(1),
        Operation::NoOp,
        Operation::Add(-13),
        Operation::Add(13),
        Operation::Add(7),
        Operation::NoOp,
        Operation::Add(1),
        Operation::Add(-33),
        Operation::NoOp,
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(2),
        Operation::NoOp,
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(8),
        Operation::NoOp,
        Operation::Add(-1),
        Operation::Add(2),
        Operation::Add(1),
        Operation::NoOp,
        Operation::Add(17),
        Operation::Add(-9),
        Operation::Add(1),
        Operation::Add(1),
        Operation::Add(-3),
        Operation::Add(11),
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(1),
        Operation::NoOp,
        Operation::Add(1),
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(-13),
        Operation::Add(-19),
        Operation::Add(1),
        Operation::Add(3),
        Operation::Add(26),
        Operation::Add(-30),
        Operation::Add(12),
        Operation::Add(-1),
        Operation::Add(3),
        Operation::Add(1),
        Operation::NoOp,
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(-9),
        Operation::Add(18),
        Operation::Add(1),
        Operation::Add(2),
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(9),
        Operation::NoOp,
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(-1),
        Operation::Add(2),
        Operation::Add(-37),
        Operation::Add(1),
        Operation::Add(3),
        Operation::NoOp,
        Operation::Add(15),
        Operation::Add(-21),
        Operation::Add(22),
        Operation::Add(-6),
        Operation::Add(1),
        Operation::NoOp,
        Operation::Add(2),
        Operation::Add(1),
        Operation::NoOp,
        Operation::Add(-10),
        Operation::NoOp,
        Operation::NoOp,
        Operation::Add(20),
        Operation::Add(1),
        Operation::Add(2),
        Operation::Add(2),
        Operation::Add(-6),
        Operation::Add(-11),
        Operation::NoOp,
        Operation::NoOp,
        Operation::NoOp,
    ];

    #[test]
    fn should_solve_first_part_example() {
        assert_eq!(super::solve_first_part(&EXAMPLE.to_vec()), 13140);
    }

    #[test]
    fn should_solve_second_part_example() {
        assert_eq!(
            super::solve_second_part(&EXAMPLE.to_vec()),
                   "##..##..##..##..##..##..##..##..##..##..\n\
                    ###...###...###...###...###...###...###.\n\
                    ####....####....####....####....####....\n\
                    #####.....#####.....#####.....#####.....\n\
                    ######......######......######......####\n\
                    #######.......#######.......#######.....\n"
        );
    }
}

