use super::Solver;
mod input;
use input::INPUT;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
struct Coordinate(i64, i64);

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

type SensorAndBeacons = Vec<(Coordinate, Coordinate)>;

pub struct Solver2022_15 {
    sensor_and_beacons: SensorAndBeacons,
}

impl Default for Solver2022_15 {
    fn default() -> Self {
        Self::from(INPUT)
    }
}

impl<'a> From<&'a str> for Solver2022_15 {
    fn from(input: &'a str) -> Self {
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        Self {
            sensor_and_beacons: input
                .split("\n")
                .map(|row| {
                    let positions = re.captures(row).unwrap();
                    (
                        Coordinate(
                            positions.get(1).unwrap().as_str().parse().unwrap(),
                            positions.get(2).unwrap().as_str().parse().unwrap(),
                        ),
                        Coordinate(
                            positions.get(3).unwrap().as_str().parse().unwrap(),
                            positions.get(4).unwrap().as_str().parse().unwrap(),
                        ),
                    )
                })
                .collect(),
        }
    }
}

impl Solver2022_15 {
    fn find_beacon_free_cells(&self, row: i64) -> usize {
        let mut beacons: HashSet<i64> = HashSet::new();
        let mut results: HashSet<i64> = HashSet::new();
        for (sensor, beacon) in &self.sensor_and_beacons {
            if beacon.1 == row {
                beacons.insert(beacon.0);
            }
            let distance = sensor.distance(beacon);
            let row_distance = (sensor.1 - row).abs();
            if row_distance < distance {
                let diff = distance - row_distance;
                for i in sensor.0 - diff..=sensor.0 + diff {
                    results.insert(i);
                }
            }
        }
        for beacon in beacons {
            results.remove(&beacon);
        }
        results.len()
    }

    fn find_distress_beacon(&self, max_coordinate: i64) -> Coordinate {
        let coverage_areas: Vec<(&Coordinate, i64)> = self
            .sensor_and_beacons
            .iter()
            .map(|(sensor, beacon)| (sensor, sensor.distance(beacon)))
            .collect();

        for j in 0..=max_coordinate {
            let mut i = 0;
            while i < max_coordinate {
                let current_location = Coordinate(i, j);
                let mut covered = false;
                for &(sensor, radius) in &coverage_areas {
                    let distance = sensor.distance(&current_location);
                    if distance <= radius {
                        i += radius - distance;
                        covered = true;
                        break;
                    }
                }
                if !covered {
                    return current_location;
                }
                i += 1;
            }
        }
        panic!("beacon not found");
    }
}

impl Solver<usize, i64> for Solver2022_15 {
    fn solve_first_part(&self) -> usize {
        self.find_beacon_free_cells(2000000)
    }

    fn solve_second_part(&self) -> i64 {
        let beacon = self.find_distress_beacon(4000000);
        (beacon.0 * 4000000) + beacon.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
        Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
        Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
        Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
        Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
        Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
        Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
        Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
        Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
        Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
        Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
        Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
        Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
        Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn should_solve_first_part() {
        let solver = Solver2022_15::from(EXAMPLE);
        assert_eq!(solver.find_beacon_free_cells(10), 26);
    }

    #[test]
    fn should_solve_second_part() {
        let solver = Solver2022_15::from(EXAMPLE);
        assert_eq!(solver.find_distress_beacon(20), Coordinate(14, 11));
    }
}
