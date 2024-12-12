use super::Solver;
mod input;
use input::INPUT;

#[derive(Clone, Debug)]
enum RegionWithPointer {
    // first number is the area, second number is the perimeter
    Actual(Region),
    Pointer(usize),
}

impl RegionWithPointer {
    fn as_mut_region(&mut self) -> &mut Region {
        match self {
            RegionWithPointer::Actual(r) => r,
            RegionWithPointer::Pointer(_) => panic!("not actual"),
        }
    }
    fn as_region(&self) -> &Region {
        match self {
            RegionWithPointer::Actual(r) => r,
            RegionWithPointer::Pointer(_) => panic!("not actual"),
        }
    }
}

#[derive(Clone, Debug)]
struct Region {
    area: usize,
    perimeter: usize,
    corners: usize,
}

impl Region {
    fn new(area: usize, perimeter: usize) -> Self {
        Self {
            area,
            perimeter,
            corners: 0,
        }
    }
}
#[derive(Clone)]
pub struct Solver2024_12 {
    input: Vec<Vec<char>>,
}

impl Default for Solver2024_12 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

impl From<&str> for Solver2024_12 {
    fn from(input: &str) -> Self {
        Self {
            input: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

fn get_region_index_from_index(regions: &Vec<RegionWithPointer>, index: usize) -> Option<usize> {
    match regions.get(index)? {
        RegionWithPointer::Actual(_) => Some(index),
        RegionWithPointer::Pointer(pointer) => {
            get_region_index_from_index(regions, pointer.clone())
        }
    }
}

fn get_region_index(
    region_map: &Vec<Vec<usize>>,
    regions: &mut Vec<RegionWithPointer>,
    x: usize,
    y: usize,
) -> Option<usize> {
    get_region_index_from_index(regions, *region_map.get(y)?.get(x)?)
}

impl Solver2024_12 {
    fn are_cells_same_region(&self, position: (usize, usize), direction: (isize, isize)) -> bool {
        let (x, y) = position;
        let (dx, dy) = direction;
        let wrapper = || -> Option<()> {
            let other_cell = self
                .input
                .get(y.checked_add_signed(dy)?)?
                .get(x.checked_add_signed(dx)?)?;
            let current_cell = self.input.get(y)?.get(x)?;
            if other_cell == current_cell {
                Some(())
            } else {
                None
            }
        };
        wrapper().is_some()
    }

    fn get_number_of_corners(&self, position: (usize, usize)) -> usize {
        let is_up_cell_same_region = self.are_cells_same_region(position, (0, -1));
        let is_left_cell_same_region = self.are_cells_same_region(position, (-1, 0));
        let is_down_cell_same_region = self.are_cells_same_region(position, (0, 1));
        let is_right_cell_same_region = self.are_cells_same_region(position, (1, 0));
        let is_up_left_cell_same_region = self.are_cells_same_region(position, (-1, -1));
        let is_up_right_cell_same_region = self.are_cells_same_region(position, (1, -1));
        let is_down_right_cell_same_region = self.are_cells_same_region(position, (1, 1));
        let is_down_left_cell_same_region = self.are_cells_same_region(position, (-1, 1));
        vec![
            !is_up_cell_same_region && !is_left_cell_same_region,
            !is_up_cell_same_region && !is_right_cell_same_region,
            !is_down_cell_same_region && !is_left_cell_same_region,
            !is_down_cell_same_region && !is_right_cell_same_region,
            !is_up_left_cell_same_region && is_left_cell_same_region && is_up_cell_same_region,
            !is_up_right_cell_same_region && is_right_cell_same_region && is_up_cell_same_region,
            !is_down_right_cell_same_region
                && is_right_cell_same_region
                && is_down_cell_same_region,
            !is_down_left_cell_same_region && is_left_cell_same_region && is_down_cell_same_region,
        ]
        .iter()
        .filter(|&&b| b)
        .count()
    }

    fn calculate_regions(&self) -> Vec<RegionWithPointer> {
        let mut region_map: Vec<Vec<usize>> = vec![];
        let mut regions: Vec<RegionWithPointer> = vec![];
        let height = self.input.len();
        let width = self.input[0].len();
        for y in 0..height {
            region_map.push(vec![]);
            for x in 0..width {
                let position = (x, y);
                let is_left_cell_same_region = self.are_cells_same_region(position, (-1, 0));
                let is_up_cell_same_region = self.are_cells_same_region(position, (0, -1));
                if is_up_cell_same_region && is_left_cell_same_region {
                    let left_region_index =
                        get_region_index(&region_map, &mut regions, x - 1, y).unwrap();
                    let up_region_index =
                        get_region_index(&region_map, &mut regions, x, y - 1).unwrap();
                    region_map[y].push(left_region_index);

                    if left_region_index == up_region_index {
                        let left_region = regions[left_region_index].as_mut_region();
                        left_region.area += 1;
                    } else {
                        let up_region_area = regions[up_region_index].as_region().area;
                        let up_region_perimeter = regions[up_region_index].as_region().perimeter;
                        let up_region_corners = regions[up_region_index].as_region().corners;
                        let left_region = regions[left_region_index].as_mut_region();
                        // left area + up area + current cell
                        left_region.area += up_region_area + 1;
                        left_region.perimeter += up_region_perimeter;
                        left_region.corners += up_region_corners;
                        regions[up_region_index] = RegionWithPointer::Pointer(left_region_index);
                    }
                } else if is_left_cell_same_region {
                    let left_region_index =
                        get_region_index(&region_map, &mut regions, x - 1, y).unwrap();
                    region_map[y].push(left_region_index);
                    let left_region = regions[left_region_index].as_mut_region();
                    left_region.perimeter += 1;
                    left_region.area += 1;
                    if let Some(index) = y
                        .checked_sub(1)
                        .and_then(|y| get_region_index(&region_map, &mut regions, x, y))
                    {
                        let region = regions[index].as_mut_region();
                        region.perimeter += 1;
                    }
                } else if is_up_cell_same_region {
                    let up_region_index =
                        get_region_index(&region_map, &mut regions, x, y - 1).unwrap();
                    let up_region = regions[up_region_index].as_mut_region();
                    region_map[y].push(up_region_index);
                    up_region.perimeter += 1;
                    up_region.area += 1;
                    if let Some(index) = x
                        .checked_sub(1)
                        .and_then(|x| get_region_index(&region_map, &mut regions, x, y))
                    {
                        let region = regions[index].as_mut_region();
                        region.perimeter += 1;
                    }
                } else {
                    region_map[y].push(regions.len());
                    regions.push(RegionWithPointer::Actual(Region::new(1, 2)));
                    if let Some(index) = x
                        .checked_sub(1)
                        .and_then(|x| get_region_index(&region_map, &mut regions, x, y))
                    {
                        let region = regions[index].as_mut_region();
                        region.perimeter += 1;
                    }
                    if let Some(index) = y
                        .checked_sub(1)
                        .and_then(|y| get_region_index(&region_map, &mut regions, x, y))
                    {
                        let region = regions[index].as_mut_region();
                        region.perimeter += 1;
                    }
                }
                let last_cell_region_index =
                    get_region_index(&region_map, &mut regions, x, y).unwrap();
                let last_cell_region = regions[last_cell_region_index].as_mut_region();
                last_cell_region.corners += self.get_number_of_corners(position);
            }
            let last_cell_region_index =
                get_region_index(&region_map, &mut regions, width - 1, y).unwrap();
            let last_cell_region = regions[last_cell_region_index].as_mut_region();
            last_cell_region.perimeter += 1;
        }
        for x in 0..width {
            let last_cell_region_index =
                get_region_index(&region_map, &mut regions, x, height - 1).unwrap();
            let last_cell_region = regions[last_cell_region_index].as_mut_region();
            last_cell_region.perimeter += 1;
        }
        regions
    }
}

impl Solver<usize, usize> for Solver2024_12 {
    fn solve_first_part(&self) -> usize {
        self.calculate_regions()
            .iter()
            .filter_map(|r| match r {
                RegionWithPointer::Pointer(_) => None,
                RegionWithPointer::Actual(region) => Some(region.area * region.perimeter),
            })
            .sum()
    }

    fn solve_second_part(&self) -> usize {
        self.calculate_regions()
            .iter()
            .filter_map(|r| match r {
                RegionWithPointer::Pointer(_) => None,
                RegionWithPointer::Actual(region) => Some(region.area * region.corners),
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn should_solve_first_part() {
        let solver = Solver2024_12::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 1930);
    }

    #[test]
    fn should_solve_second_part() {
        let solver = Solver2024_12::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 1206);
    }

    #[test]
    fn should_solve_second_part_e_shape() {
        let solver = Solver2024_12::from(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        );
        assert_eq!(solver.solve_second_part(), 236);
    }

    #[test]
    fn should_solve_second_part_two_inner_squares() {
        let solver = Solver2024_12::from(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        );
        assert_eq!(solver.solve_second_part(), 368);
    }
}
