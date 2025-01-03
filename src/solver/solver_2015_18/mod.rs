use super::Solver;

pub struct Solver2015_18 {
    number_of_steps: usize,
    lights: Vec<Vec<bool>>,
}

impl Default for Solver2015_18 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2015_18 {
    fn from(input: &str) -> Self {
        Self {
            number_of_steps: 100,
            lights: input
                .lines()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect(),
        }
    }
}

fn get_next_state(lights: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    let neighbors = (-1..=1).flat_map(|dx| {
        (-1..=1).filter_map(move |dy| {
            let ny = y.checked_add_signed(dy)?;
            let nx = x.checked_add_signed(dx)?;
            if nx == x && ny == y {
                return None;
            }
            lights.get(ny)?.get(nx).copied()
        })
    });
    let open_neighbors = neighbors.filter(|&light| light).count();
    if lights[y][x] {
        open_neighbors == 2 || open_neighbors == 3
    } else {
        open_neighbors == 3
    }
}

fn process_n_steps(lights: &Vec<Vec<bool>>, n: usize, keep_corners_open: bool) -> Vec<Vec<bool>> {
    let row_len = lights[0].len();
    let col_len = lights.len();
    let mut current = lights.clone();
    let mut next = lights.clone();
    let set_corners_if_needed = |lights: &mut Vec<Vec<bool>>| {
        if !keep_corners_open {
            return;
        }
        lights[0][0] = true;
        lights[0][row_len - 1] = true;
        lights[col_len - 1][0] = true;
        lights[col_len - 1][row_len - 1] = true;
    };

    set_corners_if_needed(&mut current);
    for _ in 0..n {
        for y in 0..current.len() {
            for x in 0..current[y].len() {
                next[y][x] = get_next_state(&current, x, y);
            }
        }
        let temp = current;
        current = next;
        next = temp;
        set_corners_if_needed(&mut current);
    }
    current
}

impl Solver<usize, usize> for Solver2015_18 {
    fn solve_first_part(&self) -> usize {
        process_n_steps(&self.lights, self.number_of_steps, false)
            .iter()
            .map(|row| row.iter().filter(|light| **light).count())
            .sum()
    }

    fn solve_second_part(&self) -> usize {
        process_n_steps(&self.lights, self.number_of_steps, true)
            .iter()
            .map(|row| row.iter().filter(|light| **light).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn should_solve_first_part_example() {
        let mut solver = Solver2015_18::from(EXAMPLE);
        solver.number_of_steps = 4;
        assert_eq!(solver.solve_first_part(), 4);
    }

    #[test]
    fn should_solve_second_part_example() {
        let mut solver = Solver2015_18::from(EXAMPLE);
        solver.number_of_steps = 5;
        assert_eq!(solver.solve_second_part(), 17);
    }
}
