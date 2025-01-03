use super::Solver;

pub struct Solver2015_15 {
    ingredients: Vec<(isize, isize, isize, isize, isize)>,
}

impl Default for Solver2015_15 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2015_15 {
    fn from(input: &str) -> Self {
        let re = regex::Regex::new(r"(?P<name>\w+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)").unwrap();
        Self {
            ingredients: re
                .captures_iter(input)
                .map(|captures: regex::Captures| {
                    (
                        captures.name("capacity").unwrap().as_str().parse().unwrap(),
                        captures
                            .name("durability")
                            .unwrap()
                            .as_str()
                            .parse()
                            .unwrap(),
                        captures.name("flavor").unwrap().as_str().parse().unwrap(),
                        captures.name("texture").unwrap().as_str().parse().unwrap(),
                        captures.name("calories").unwrap().as_str().parse().unwrap(),
                    )
                })
                .collect(),
        }
    }
}

impl Solver2015_15 {
    fn get_calories(&self, ingredients: &Vec<usize>) -> isize {
        let mut calories = 0;
        for i in 0..ingredients.len() {
            calories += self.ingredients[i].4 * ingredients[i] as isize;
        }
        calories
    }
    fn get_score_for_ingredients(&self, selected_ingredients: &Vec<usize>) -> isize {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        for i in 0..self.ingredients.len() {
            let amount = selected_ingredients[i] as isize;
            capacity += self.ingredients[i].0 * amount;
            durability += self.ingredients[i].1 * amount;
            flavor += self.ingredients[i].2 * amount;
            texture += self.ingredients[i].3 * amount;
        }
        if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
            return 0;
        }
        capacity * durability * flavor * texture
    }
    fn get_best_score_with_selected_ingredients(
        &self,
        selected_ingredients: &mut Vec<usize>,
        filter_calories: bool,
    ) -> isize {
        let total_amount: usize = selected_ingredients.iter().sum();
        if selected_ingredients.len() + 1 == self.ingredients.len() {
            selected_ingredients.push(100 - total_amount);
            let result = self.get_score_for_ingredients(selected_ingredients);
            let calories = self.get_calories(selected_ingredients);
            selected_ingredients.pop();
            if filter_calories && calories != 500 {
                return isize::MIN;
            }
            return result;
        }
        let mut best_score = isize::MIN;
        for i in 1..=100 + selected_ingredients.len() + 1 - self.ingredients.len() - total_amount {
            selected_ingredients.push(i);
            let score = self
                .get_best_score_with_selected_ingredients(selected_ingredients, filter_calories);
            if score > best_score {
                best_score = score;
            }
            selected_ingredients.pop();
        }
        best_score
    }
    fn get_best_score(&self, filter_calories: bool) -> isize {
        let mut selected_ingredients = vec![];
        self.get_best_score_with_selected_ingredients(&mut selected_ingredients, filter_calories)
    }
}

impl Solver<isize, isize> for Solver2015_15 {
    fn solve_first_part(&self) -> isize {
        self.get_best_score(false)
    }

    fn solve_second_part(&self) -> isize {
        self.get_best_score(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2015_15::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 62842880);
    }

    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2015_15::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 57600000);
    }
}
