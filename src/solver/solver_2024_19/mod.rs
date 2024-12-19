use crate::solver::Solver;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Solver2024_19<'a> {
    patterns: Vec<Cow<'a, str>>,
    designs: Vec<Cow<'a, str>>,
}

impl Default for Solver2024_19<'static> {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl<'a> From<&'a str> for Solver2024_19<'a> {
    fn from(input: &'a str) -> Self {
        let parts: Vec<&str> = input.split("\n\n").collect();
        Self {
            patterns: parts[0].split(", ").map(|s| Cow::Borrowed(s)).collect(),
            designs: parts[1].lines().map(|s| Cow::Borrowed(s)).collect(),
        }
    }
}

impl<'a> Solver2024_19<'a> {
    fn is_match(&self, design: &str) -> bool {
        for pattern in &self.patterns {
            if design == pattern.as_ref()
                || design.starts_with(pattern.as_ref()) && self.is_match(&design[pattern.len()..])
            {
                return true;
            }
        }
        false
    }

    fn count_matches<'b>(&'b self, design: &'b str, cache: &mut HashMap<&'b str, usize>) -> usize {
        if let Some(count) = cache.get(design) {
            return *count;
        }
        let mut count = 0;
        for pattern in &self.patterns {
            if design == pattern.as_ref() {
                count += 1;
            } else if design.starts_with(pattern.as_ref()) {
                count += self.count_matches(&design[pattern.len()..], cache);
            }
        }
        cache.insert(design, count);
        count
    }
}

impl<'a> Solver<usize, usize> for Solver2024_19<'a> {
    fn solve_first_part(&self) -> usize {
        self.designs.iter().filter(|s| self.is_match(s)).count()
    }

    fn solve_second_part(&self) -> usize {
        self.designs
            .iter()
            .map(|s| self.count_matches(s, &mut HashMap::new()))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb\
";
    #[test]
    fn test_solve_first_part() {
        let solver = Solver2024_19::from(EXAMPLE);

        assert_eq!(solver.solve_first_part(), 6);
    }

    #[test]
    fn test_solve_second_part() {
        let solver = Solver2024_19::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 16);
    }
}
