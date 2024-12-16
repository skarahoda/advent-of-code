fn get_forest() -> Vec<Vec<u32>> {
    include_str!("input.txt")
        .lines()
        .map(|row| row.chars().map(|tree| tree.to_digit(10).unwrap()).collect())
        .collect()
}

fn solve_first_part(forest: &Vec<Vec<u32>>) -> usize {
    let row_length = forest.len();
    let column_length = forest[0].len();
    let mut visible_map = vec![vec![false; column_length]; row_length];
    for i in 0..row_length {
        let mut max_height = forest[i][0];
        visible_map[i][0] = true;
        for j in 1..column_length {
            if forest[i][j] > max_height {
                max_height = forest[i][j];
                visible_map[i][j] = true;
            }
        }
        let mut max_height = forest[i][column_length - 1];
        visible_map[i][column_length - 1] = true;
        for j in (0..column_length - 1).rev() {
            if forest[i][j] > max_height {
                max_height = forest[i][j];
                visible_map[i][j] = true;
            }
        }
    }

    for j in 0..column_length {
        let mut max_height = forest[0][j];
        visible_map[0][j] = true;
        for i in 1..row_length {
            if forest[i][j] > max_height {
                max_height = forest[i][j];
                visible_map[i][j] = true;
            }
        }
        let mut max_height = forest[row_length - 1][j];
        visible_map[row_length - 1][j] = true;
        for i in (0..row_length - 1).rev() {
            if forest[i][j] > max_height {
                max_height = forest[i][j];
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

fn solve_second_part(forest: &Vec<Vec<u32>>) -> usize {
    let row_length = forest.len();
    let column_length = forest[0].len();
    let mut scene_map = vec![vec![[0usize; 4]; column_length]; row_length];
    for i in 0..row_length {
        for j in 1..column_length {
            let current_height = forest[i][j];
            for k in (0..j).rev() {
                let neighbour_height = forest[i][k];
                if neighbour_height >= current_height || k == 0 {
                    scene_map[i][j][0] = j - k;
                    break;
                }
            }
        }
        for j in (0..column_length - 1).rev() {
            let current_height = forest[i][j];
            for k in j + 1..column_length {
                let neighbour_height = forest[i][k];
                if neighbour_height >= current_height || k == column_length - 1 {
                    scene_map[i][j][1] = k - j;
                    break;
                }
            }
        }
    }

    for j in 0..column_length {
        for i in 1..row_length {
            let current_height = forest[i][j];
            for k in (0..i).rev() {
                let neighbour_height = forest[k][j];
                if neighbour_height >= current_height || k == 0 {
                    scene_map[i][j][2] = i - k;
                    break;
                }
            }
        }
        for i in (0..row_length - 1).rev() {
            let current_height = forest[i][j];
            for k in i + 1..row_length {
                let neighbour_height = forest[k][j];
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

pub fn solve() -> (usize, usize) {
    let forest = get_forest();
    (solve_first_part(&forest), solve_second_part(&forest))
}

#[cfg(test)]
mod tests {
    static EXAMPLE: [[u32; 5]; 5] = [
        [3, 0, 3, 7, 3],
        [2, 5, 5, 1, 2],
        [6, 5, 3, 3, 2],
        [3, 3, 5, 4, 9],
        [3, 5, 3, 9, 0],
    ];
    #[test]
    fn should_solve_first_part_example() {
        assert_eq!(
            super::solve_first_part(&EXAMPLE.iter().map(|row| row.to_vec()).collect()),
            21
        );
    }

    #[test]
    fn should_solve_second_part_example() {
        assert_eq!(
            super::solve_second_part(&EXAMPLE.iter().map(|row| row.to_vec()).collect()),
            8
        );
    }
}
