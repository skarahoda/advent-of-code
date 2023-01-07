use super::utils;

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

fn get_rectangular_prisms() -> Vec<RectangularPrism> {
    let input = utils::get_input("inputs/2015_02.txt");

    input
        .split("\n")
        .map(|row| {
            let edges: Vec<i32> = row.split("x").map(|edge| edge.parse().unwrap()).collect();
            RectangularPrism::new(edges[0], edges[1], edges[2])
        })
        .collect()
}

fn solve_first_part(rectangle_prisms: &Vec<RectangularPrism>) -> i32 {
    rectangle_prisms.iter().fold(0, |score, rectangle_prism| {
        score + rectangle_prism.get_area() + rectangle_prism.get_smallest_area_of_sides()
    })
}
fn solve_second_part(rectangle_prisms: &Vec<RectangularPrism>) -> i32 {
    rectangle_prisms.iter().fold(0, |score, rectangle_prism| {
        score + rectangle_prism.get_volume() + rectangle_prism.get_smallest_perimeter_of_sides()
    })
}

pub fn solve() -> (i32, i32) {
    let rectangle_prisms = get_rectangular_prisms();
    (
        solve_first_part(&rectangle_prisms),
        solve_second_part(&rectangle_prisms),
    )
}

#[cfg(test)]
mod first_part {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(solve_first_part(&vec![RectangularPrism::new(2, 3, 4)]), 58);
    }
    #[test]
    fn second_example() {
        assert_eq!(solve_first_part(&vec![RectangularPrism::new(1, 1, 10)]), 43);
    }
}

#[cfg(test)]
mod second_part {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(solve_second_part(&vec![RectangularPrism::new(2, 3, 4)]), 34);
    }
    #[test]
    fn second_example() {
        assert_eq!(
            solve_second_part(&vec![RectangularPrism::new(1, 1, 10)]),
            14
        );
    }
}
