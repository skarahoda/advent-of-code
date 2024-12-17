use super::Solver;

pub struct Solver2022_08 {
    forest: Vec<Vec<u32>>,
}

impl Default for Solver2022_08 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2022_08 {
    fn from(input: &str) -> Self {
        Self {
            forest: input
                .lines()
                .map(|row| row.chars().map(|tree| tree.to_digit(10).unwrap()).collect())
                .collect(),
        }
    }
}

impl Solver<usize, usize> for Solver2022_08 {
    fn solve_first_part(&self) -> usize {
        let row_length = self.forest.len();
        let column_length = self.forest[0].len();
        let mut visible_map = vec![vec![false; column_length]; row_length];
        for i in 0..row_length {
            let mut max_height = self.forest[i][0];
            visible_map[i][0] = true;
            for j in 1..column_length {
                if self.forest[i][j] > max_height {
                    max_height = self.forest[i][j];
                    visible_map[i][j] = true;
                }
            }
            let mut max_height = self.forest[i][column_length - 1];
            visible_map[i][column_length - 1] = true;
            for j in (0..column_length - 1).rev() {
                if self.forest[i][j] > max_height {
                    max_height = self.forest[i][j];
                    visible_map[i][j] = true;
                }
            }
        }

        for j in 0..column_length {
            let mut max_height = self.forest[0][j];
            visible_map[0][j] = true;
            for i in 1..row_length {
                if self.forest[i][j] > max_height {
                    max_height = self.forest[i][j];
                    visible_map[i][j] = true;
                }
            }
            let mut max_height = self.forest[row_length - 1][j];
            visible_map[row_length - 1][j] = true;
            for i in (0..row_length - 1).rev() {
                if self.forest[i][j] > max_height {
                    max_height = self.forest[i][j];
                    visible_map[i][j] = true;
                }
            }
        }
        visible_map.iter().fold(0, |count, row| {
            row.iter().fold(
                count,
                |count, is_visible| if *is_visible { count + 1 } else { count },
            )
        })
    }

    fn solve_second_part(&self) -> usize {
        let row_length = self.forest.len();
        let column_length = self.forest[0].len();
        let mut scene_map = vec![vec![[0usize; 4]; column_length]; row_length];
        for i in 0..row_length {
            for j in 1..column_length {
                let current_height = self.forest[i][j];
                for k in (0..j).rev() {
                    let neighbour_height = self.forest[i][k];
                    if neighbour_height >= current_height || k == 0 {
                        scene_map[i][j][0] = j - k;
                        break;
                    }
                }
            }
            for j in (0..column_length - 1).rev() {
                let current_height = self.forest[i][j];
                for k in j + 1..column_length {
                    let neighbour_height = self.forest[i][k];
                    if neighbour_height >= current_height || k == column_length - 1 {
                        scene_map[i][j][1] = k - j;
                        break;
                    }
                }
            }
        }

        for j in 0..column_length {
            for i in 1..row_length {
                let current_height = self.forest[i][j];
                for k in (0..i).rev() {
                    let neighbour_height = self.forest[k][j];
                    if neighbour_height >= current_height || k == 0 {
                        scene_map[i][j][2] = i - k;
                        break;
                    }
                }
            }
            for i in (0..row_length - 1).rev() {
                let current_height = self.forest[i][j];
                for k in i + 1..row_length {
                    let neighbour_height = self.forest[k][j];
                    if neighbour_height >= current_height || k == column_length - 1 {
                        scene_map[i][j][3] = k - i;
                        break;
                    }
                }
            }
        }

        scene_map.iter().fold(0, |highest_score, row| {
            row.iter().fold(highest_score, |highest_score, item| {
                std::cmp::max(item[0] * item[1] * item[2] * item[3], highest_score)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE: &str = "\
30373
25512
65332
33549
35390\
";
    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2022_08::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 21);
    }

    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2022_08::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 8);
    }
}
