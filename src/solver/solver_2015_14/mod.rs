use super::Solver;

pub struct Solver2015_14 {
    duration: usize,
    reindeer: Vec<(usize, usize, usize)>,
}

impl Default for Solver2015_14 {
    fn default() -> Self {
        Self::from(include_str!("input.txt"))
    }
}

impl From<&str> for Solver2015_14 {
    fn from(input: &str) -> Self {
        let re_duration = regex::Regex::new(r"Duration: (?P<duration>\d+) seconds").unwrap();
        let re_reindeer = regex::Regex::new(r"(?P<name>\w+) can fly (?P<speed>\d+) km/s for (?P<duration>\d+) seconds, but then must rest for (?P<rest>\d+) seconds.").unwrap();
        Self {
            duration: re_duration
                .captures(input)
                .unwrap()
                .name("duration")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            reindeer: re_reindeer
                .captures_iter(input)
                .map(|captures: regex::Captures| {
                    (
                        captures.name("speed").unwrap().as_str().parse().unwrap(),
                        captures.name("duration").unwrap().as_str().parse().unwrap(),
                        captures.name("rest").unwrap().as_str().parse().unwrap(),
                    )
                })
                .collect(),
        }
    }
}

fn get_total_distance(
    duration: &usize,
    speed: &usize,
    move_duration: &usize,
    rest_duration: &usize,
) -> usize {
    let round_duration = move_duration + rest_duration;
    let last_round_duration = duration % round_duration;
    let rounds = duration / round_duration;
    let last_round_move_duration = usize::min(*move_duration, last_round_duration);
    speed * ((rounds * move_duration) + last_round_move_duration)
}

impl Solver2015_14 {
    fn get_winner_score(&self) -> usize {
        let mut distances: Vec<usize> = vec![0; self.reindeer.len()];
        let mut scores: Vec<usize> = vec![0; self.reindeer.len()];
        for i in 0..self.duration {
            for (index, &(speed, move_duration, rest_duration)) in self.reindeer.iter().enumerate()
            {
                let round_duration = move_duration + rest_duration;
                let duration_in_last_round = i % round_duration;
                if duration_in_last_round < move_duration {
                    distances[index] += speed;
                }
            }
            let winner_score = distances.iter().max().unwrap();
            for (index, distance) in distances.iter().enumerate() {
                if distance == winner_score {
                    scores[index] += 1;
                }
            }
        }
        scores.iter().max().unwrap().clone()
    }
}

impl Solver<usize, usize> for Solver2015_14 {
    fn solve_first_part(&self) -> usize {
        self.reindeer
            .iter()
            .map(|(speed, move_duration, rest_duration)| {
                get_total_distance(&self.duration, speed, move_duration, rest_duration)
            })
            .max()
            .unwrap()
    }

    fn solve_second_part(&self) -> usize {
        self.get_winner_score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn should_solve_first_part_example() {
        let solver = Solver2015_14::from(EXAMPLE);
        assert_eq!(solver.solve_first_part(), 1120);
    }

    #[test]
    fn should_solve_second_part_example() {
        let solver = Solver2015_14::from(EXAMPLE);
        assert_eq!(solver.solve_second_part(), 689);
    }
}
