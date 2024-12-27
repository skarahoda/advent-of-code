use super::Solver;

struct RectangularPrism {
    width: i32,
    length: i32,
    height: i32,
}

impl RectangularPrism {
    fn new(width: i32, length: i32, height: i32) -> Self {
        RectangularPrism {
            width,
            length,
            height,
        }
    }

    fn get_area(&self) -> i32 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    fn get_smallest_area_of_sides(&self) -> i32 {
        let area1 = self.length * self.width;
        let area2 = self.width * self.height;
        let area3 = self.height * self.length;
        *vec![area1, area2, area3].iter().min().unwrap()
    }

    fn get_smallest_perimeter_of_sides(&self) -> i32 {
        let perimeter1 = 2 * (self.length + self.width);
        let perimeter2 = 2 * (self.width + self.height);
        let perimeter3 = 2 * (self.height + self.length);
        *vec![perimeter1, perimeter2, perimeter3]
            .iter()
            .min()
            .unwrap()
    }

    fn get_volume(&self) -> i32 {
        self.width * self.length * self.height
    }
}

pub struct Solver2015_02 {
    rectangle_prisms: Vec<RectangularPrism>,
}

impl Default for Solver2015_02 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}
impl From<&str> for Solver2015_02 {
    fn from(input: &str) -> Self {
        Self {
            rectangle_prisms: input
                .split("\n")
                .map(|row| {
                    let edges: Vec<i32> =
                        row.split("x").map(|edge| edge.parse().unwrap()).collect();
                    RectangularPrism::new(edges[0], edges[1], edges[2])
                })
                .collect(),
        }
    }
}

impl Solver<i32, i32> for Solver2015_02 {
    fn solve_first_part(&self) -> i32 {
        self.rectangle_prisms
            .iter()
            .map(|rectangle_prism| {
                rectangle_prism.get_area() + rectangle_prism.get_smallest_area_of_sides()
            })
            .sum()
    }
    fn solve_second_part(&self) -> i32 {
        self.rectangle_prisms
            .iter()
            .map(|rectangle_prism| {
                rectangle_prism.get_volume() + rectangle_prism.get_smallest_perimeter_of_sides()
            })
            .sum()
    }
}

#[cfg(test)]
mod first_part {
    use super::*;

    #[test]
    fn first_example() {
        let solver = Solver2015_02::from("2x3x4");
        assert_eq!(solver.solve_first_part(), 58);
    }
    #[test]
    fn second_example() {
        let solver = Solver2015_02::from("1x1x10");
        assert_eq!(solver.solve_first_part(), 43);
    }
}

#[cfg(test)]
mod second_part {
    use super::*;

    #[test]
    fn first_example() {
        let solver = Solver2015_02::from("2x3x4");
        assert_eq!(solver.solve_second_part(), 34);
    }
    #[test]
    fn second_example() {
        let solver = Solver2015_02::from("1x1x10");
        assert_eq!(solver.solve_second_part(), 14);
    }
}
