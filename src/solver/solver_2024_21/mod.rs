use crate::solver::Solver;
use std::fmt::Display;
use std::ops::Sub;

#[derive(Clone, Copy)]
struct Vector2D(isize, isize);

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Clone, Copy)]
enum DirectionalKeyPadButton {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl Display for DirectionalKeyPadButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionalKeyPadButton::Up => write!(f, "^"),
            DirectionalKeyPadButton::Down => write!(f, "v"),
            DirectionalKeyPadButton::Left => write!(f, "<"),
            DirectionalKeyPadButton::Right => write!(f, ">"),
            DirectionalKeyPadButton::A => write!(f, "A"),
        }
    }
}

impl From<DirectionalKeyPadButton> for Vector2D {
    fn from(direction: DirectionalKeyPadButton) -> Self {
        match direction {
            DirectionalKeyPadButton::Up => Vector2D(1, 0),
            DirectionalKeyPadButton::A => Vector2D(2, 0),
            DirectionalKeyPadButton::Left => Vector2D(0, 1),
            DirectionalKeyPadButton::Down => Vector2D(1, 1),
            DirectionalKeyPadButton::Right => Vector2D(2, 1),
        }
    }
}

impl Sub<DirectionalKeyPadButton> for DirectionalKeyPadButton {
    type Output = Vec<DirectionalKeyPadButton>;

    fn sub(self, rhs: DirectionalKeyPadButton) -> Self::Output {
        let self_vec = Vector2D::from(self);
        let rhs_vec = Vector2D::from(rhs);
        let distance = self_vec - rhs_vec;
        let mut result = vec![];

        if self_vec.0 == 0 {
            result.extend(vec![
                DirectionalKeyPadButton::Down;
                distance.1.unsigned_abs()
            ]);
            result.extend(vec![
                DirectionalKeyPadButton::Left;
                distance.0.unsigned_abs()
            ]);
        } else if rhs_vec.0 == 0 {
            result.extend(vec![
                DirectionalKeyPadButton::Right;
                distance.0.unsigned_abs()
            ]);
            result.extend(vec![DirectionalKeyPadButton::Up; distance.1.unsigned_abs()]);
        } else {
            if distance.0 < 0 {
                result.extend(vec![
                    DirectionalKeyPadButton::Left;
                    distance.0.unsigned_abs()
                ]);
            }
            if distance.1 < 0 {
                result.extend(vec![DirectionalKeyPadButton::Up; distance.1.unsigned_abs()]);
            }
            if distance.1 > 0 {
                result.extend(vec![
                    DirectionalKeyPadButton::Down;
                    distance.1.unsigned_abs()
                ]);
            }
            if distance.0 > 0 {
                result.extend(vec![
                    DirectionalKeyPadButton::Right;
                    distance.0.unsigned_abs()
                ]);
            }
        }
        result
    }
}

#[derive(Clone, Copy)]
enum NumericKeyPadButton {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

impl From<char> for NumericKeyPadButton {
    fn from(c: char) -> Self {
        match c {
            '0' => NumericKeyPadButton::Zero,
            '1' => NumericKeyPadButton::One,
            '2' => NumericKeyPadButton::Two,
            '3' => NumericKeyPadButton::Three,
            '4' => NumericKeyPadButton::Four,
            '5' => NumericKeyPadButton::Five,
            '6' => NumericKeyPadButton::Six,
            '7' => NumericKeyPadButton::Seven,
            '8' => NumericKeyPadButton::Eight,
            '9' => NumericKeyPadButton::Nine,
            'A' => NumericKeyPadButton::A,
            _ => unreachable!(),
        }
    }
}

impl From<NumericKeyPadButton> for Vector2D {
    fn from(key: NumericKeyPadButton) -> Self {
        match key {
            NumericKeyPadButton::Seven => Vector2D(0, 0),
            NumericKeyPadButton::Eight => Vector2D(1, 0),
            NumericKeyPadButton::Nine => Vector2D(2, 0),
            NumericKeyPadButton::Four => Vector2D(0, 1),
            NumericKeyPadButton::Five => Vector2D(1, 1),
            NumericKeyPadButton::Six => Vector2D(2, 1),
            NumericKeyPadButton::One => Vector2D(0, 2),
            NumericKeyPadButton::Two => Vector2D(1, 2),
            NumericKeyPadButton::Three => Vector2D(2, 2),
            NumericKeyPadButton::Zero => Vector2D(1, 3),
            NumericKeyPadButton::A => Vector2D(2, 3),
        }
    }
}

impl Sub<NumericKeyPadButton> for NumericKeyPadButton {
    type Output = Vec<DirectionalKeyPadButton>;

    fn sub(self, rhs: NumericKeyPadButton) -> Self::Output {
        let self_vec = Vector2D::from(self);
        let rhs_vec = Vector2D::from(rhs);
        let distance = self_vec - rhs_vec;
        let mut result = vec![];

        if rhs_vec.1 == 3 && self_vec.0 == 0 {
            result.extend(vec![DirectionalKeyPadButton::Up; distance.1.unsigned_abs()]);
            result.extend(vec![
                DirectionalKeyPadButton::Left;
                distance.0.unsigned_abs()
            ]);
        } else if rhs_vec.0 == 0 && self_vec.1 == 3 {
            result.extend(vec![
                DirectionalKeyPadButton::Right;
                distance.0.unsigned_abs()
            ]);
            result.extend(vec![
                DirectionalKeyPadButton::Down;
                distance.1.unsigned_abs()
            ]);
        } else {
            if distance.0 < 0 {
                result.extend(vec![
                    DirectionalKeyPadButton::Left;
                    distance.0.unsigned_abs()
                ]);
            }
            if distance.1 < 0 {
                result.extend(vec![DirectionalKeyPadButton::Up; distance.1.unsigned_abs()]);
            }
            if distance.1 > 0 {
                result.extend(vec![
                    DirectionalKeyPadButton::Down;
                    distance.1.unsigned_abs()
                ]);
            }
            if distance.0 > 0 {
                result.extend(vec![
                    DirectionalKeyPadButton::Right;
                    distance.0.unsigned_abs()
                ]);
            }
        }
        result
    }
}

pub struct Solver2024_21 {
    door_codes: Vec<Vec<NumericKeyPadButton>>,
}

impl Default for Solver2024_21 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2024_21 {
    fn from(input: &str) -> Self {
        Self {
            door_codes: input
                .lines()
                .map(|line| line.chars().map(NumericKeyPadButton::from).collect())
                .collect(),
        }
    }
}

impl Solver2024_21 {
    fn get_direction_keys_from_door_code(
        &self,
        door_code: &Vec<NumericKeyPadButton>,
    ) -> Vec<DirectionalKeyPadButton> {
        let mut current_code_key = NumericKeyPadButton::A;
        let mut result: Vec<DirectionalKeyPadButton> = vec![];
        for &button in door_code {
            result.extend(button - current_code_key);
            result.push(DirectionalKeyPadButton::A);
            current_code_key = button;
        }
        result
    }

    fn get_direction_keys_from_direction_keys(
        &self,
        direction_keys: &Vec<DirectionalKeyPadButton>,
    ) -> Vec<DirectionalKeyPadButton> {
        let mut current_direction_key = DirectionalKeyPadButton::A;
        let mut result: Vec<DirectionalKeyPadButton> = vec![];
        for &direction_key in direction_keys {
            result.extend(direction_key - current_direction_key);
            result.push(DirectionalKeyPadButton::A);
            current_direction_key = direction_key;
        }
        result
    }
}

impl Solver<usize, usize> for Solver2024_21 {
    fn solve_first_part(&self) -> usize {
        self.door_codes
            .iter()
            .map(|door_code| {
                let first_robot_keys = self.get_direction_keys_from_door_code(door_code);
                let second_robot_keys =
                    self.get_direction_keys_from_direction_keys(&first_robot_keys);
                let player_keys = self.get_direction_keys_from_direction_keys(&second_robot_keys);
                let num = door_code
                    .iter()
                    .filter_map(|button| match button {
                        NumericKeyPadButton::Zero => Some(0),
                        NumericKeyPadButton::One => Some(1),
                        NumericKeyPadButton::Two => Some(2),
                        NumericKeyPadButton::Three => Some(3),
                        NumericKeyPadButton::Four => Some(4),
                        NumericKeyPadButton::Five => Some(5),
                        NumericKeyPadButton::Six => Some(6),
                        NumericKeyPadButton::Seven => Some(7),
                        NumericKeyPadButton::Eight => Some(8),
                        NumericKeyPadButton::Nine => Some(9),
                        _ => None,
                    })
                    .fold(0, |acc, x| acc * 10 + x);
                player_keys.len() * num
            })
            .sum()
    }

    fn solve_second_part(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn test_solve_first_part() {
        let mut solver = Solver2024_21::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 126384);
    }

    #[test]
    fn test_solve_second_part() {
        let mut solver = Solver2024_21::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 0);
    }
}
