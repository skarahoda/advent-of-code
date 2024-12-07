mod input;
use input::INPUT;
use std::cmp::{max, min};
use std::collections::HashSet;

type Coordinates = (usize, usize, usize);

fn get_max(droplets: &Vec<Coordinates>) -> Coordinates {
    droplets
        .iter()
        .fold(*droplets.first().unwrap(), |acc, current| {
            (
                max(acc.0, current.0),
                max(acc.1, current.1),
                max(acc.2, current.2),
            )
        })
}

fn get_min(droplets: &Vec<Coordinates>) -> Coordinates {
    droplets
        .iter()
        .fold(*droplets.first().unwrap(), |acc, current| {
            (
                min(acc.0, current.0),
                min(acc.1, current.1),
                min(acc.2, current.2),
            )
        })
}

fn subtract(a: Coordinates, b: Coordinates) -> Coordinates {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn get_droplets(input: &str) -> Vec<Coordinates> {
    input
        .split("\n")
        .map(|coordinate| {
            let coordinate: Vec<usize> =
                coordinate.split(",").map(|i| i.parse().unwrap()).collect();
            (coordinate[0], coordinate[1], coordinate[2])
        })
        .collect()
}

fn solve_first_part(droplets: &Vec<Coordinates>) -> usize {
    let droplet_set: HashSet<Coordinates> = HashSet::from_iter(droplets.iter().map(|&c| c));
    let mut result = droplets.len() * 6;
    for droplet in droplets {
        if droplet_set.contains(&(droplet.0 + 1, droplet.1, droplet.2)) {
            result -= 2;
        }
        if droplet_set.contains(&(droplet.0, droplet.1 + 1, droplet.2)) {
            result -= 2;
        }
        if droplet_set.contains(&(droplet.0, droplet.1, droplet.2 + 1)) {
            result -= 2;
        }
    }
    result
}

fn solve_second_part(droplets: &Vec<Coordinates>) -> usize {
    let max_coordinates = get_max(droplets);
    let min_coordinates = get_min(droplets);
    let reduced_max_coordinates = subtract(max_coordinates, min_coordinates);
    let droplet_set: HashSet<Coordinates> =
        HashSet::from_iter(droplets.iter().map(|&c| subtract(c, min_coordinates)));
    let mut group_map =
        vec![
            vec![vec![0; reduced_max_coordinates.2 + 1]; reduced_max_coordinates.1 + 1];
            reduced_max_coordinates.0 + 1
        ];
    let mut neighbour_map: HashSet<(usize, usize)> = HashSet::new();
    let mut outer_groups: HashSet<usize> = HashSet::new();
    let mut next_group = 1;
    for i in 0..=reduced_max_coordinates.0 {
        for j in 0..=reduced_max_coordinates.1 {
            for k in 0..=reduced_max_coordinates.2 {
                if droplet_set.contains(&(i, j, k)) {
                    continue;
                }
                let mut neighbours: HashSet<usize> = HashSet::new();
                if k > 0 && !droplet_set.contains(&(i, j, k - 1)) {
                    neighbours.insert(group_map[i][j][k - 1]);
                }
                if j > 0 && !droplet_set.contains(&(i, j - 1, k)) {
                    neighbours.insert(group_map[i][j - 1][k]);
                }
                if i > 0 && !droplet_set.contains(&(i - 1, j, k)) {
                    neighbours.insert(group_map[i - 1][j][k]);
                }
                if let Some(&group) = neighbours.iter().next() {
                    group_map[i][j][k] = group;
                } else {
                    group_map[i][j][k] = next_group;
                    next_group += 1;
                }
                let current_group = group_map[i][j][k];
                for neighbour in neighbours {
                    if neighbour == current_group {
                        continue;
                    }
                    neighbour_map.insert(if current_group > neighbour {
                        (neighbour, current_group)
                    } else {
                        (current_group, neighbour)
                    });
                }
                if i == 0
                    || i == reduced_max_coordinates.0
                    || j == 0
                    || j == reduced_max_coordinates.1
                    || k == 0
                    || k == reduced_max_coordinates.2
                {
                    outer_groups.insert(current_group);
                }
            }
        }
    }

    let mut queue: Vec<usize> = outer_groups.iter().map(|&group| group).collect();

    while let Some(current_group) = queue.pop() {
        for neighbour in 0..next_group {
            let neighbour_pair = if current_group > neighbour {
                (neighbour, current_group)
            } else {
                (current_group, neighbour)
            };
            if !outer_groups.contains(&neighbour) && neighbour_map.contains(&neighbour_pair) {
                outer_groups.insert(neighbour);
                queue.push(neighbour);
            }
        }
    }

    let mut result = 0;
    for (x, y, z) in droplet_set {
        if x == 0 || outer_groups.contains(&group_map[x - 1][y][z]) {
            result += 1;
        }
        if y == 0 || outer_groups.contains(&group_map[x][y - 1][z]) {
            result += 1;
        }
        if z == 0 || outer_groups.contains(&group_map[x][y][z - 1]) {
            result += 1;
        }
        if x == reduced_max_coordinates.0 || outer_groups.contains(&group_map[x + 1][y][z]) {
            result += 1;
        }
        if y == reduced_max_coordinates.1 || outer_groups.contains(&group_map[x][y + 1][z]) {
            result += 1;
        }
        if z == reduced_max_coordinates.2 || outer_groups.contains(&group_map[x][y][z + 1]) {
            result += 1;
        }
    }
    result
}

pub fn solve() -> (usize, usize) {
    let droplets = get_droplets(INPUT);
    (solve_first_part(&droplets), solve_second_part(&droplets))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
        2,2,2\n\
        1,2,2\n\
        3,2,2\n\
        2,1,2\n\
        2,3,2\n\
        2,2,1\n\
        2,2,3\n\
        2,2,4\n\
        2,2,6\n\
        1,2,5\n\
        3,2,5\n\
        2,1,5\n\
        2,3,5\
    ";

    #[test]
    fn should_get_droplets() {
        assert_eq!(
            get_droplets(EXAMPLE),
            vec![
                (2, 2, 2),
                (1, 2, 2),
                (3, 2, 2),
                (2, 1, 2),
                (2, 3, 2),
                (2, 2, 1),
                (2, 2, 3),
                (2, 2, 4),
                (2, 2, 6),
                (1, 2, 5),
                (3, 2, 5),
                (2, 1, 5),
                (2, 3, 5),
            ]
        );
    }

    #[test]
    fn should_solve_first_part() {
        assert_eq!(solve_first_part(&get_droplets(EXAMPLE)), 64);
    }

    #[test]
    fn should_solve_second_part() {
        assert_eq!(solve_second_part(&get_droplets(EXAMPLE)), 58);
    }
}
