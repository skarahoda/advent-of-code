use super::utils;
use regex::Regex;
use std::collections::HashSet;

type Coordinates = (i64, i64);
type SensorAndBeacons = Vec<(Coordinates, Coordinates)>;

fn get_sensor_and_beacons(input: &str) -> SensorAndBeacons {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    input
        .split("\n")
        .map(|row| {
            let positions = re.captures(row).unwrap();
            (
                (
                    positions.get(1).unwrap().as_str().parse().unwrap(),
                    positions.get(2).unwrap().as_str().parse().unwrap(),
                ),
                (
                    positions.get(3).unwrap().as_str().parse().unwrap(),
                    positions.get(4).unwrap().as_str().parse().unwrap(),
                ),
            )
        })
        .collect()
}

fn find_distance(a: &Coordinates, b: &Coordinates) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn find_beacon_free_cells(sensor_and_beacons: &SensorAndBeacons, row: i64) -> usize {
    let mut beacons: HashSet<i64> = HashSet::new();
    let mut results: HashSet<i64> = HashSet::new();
    for (sensor, beacon) in sensor_and_beacons {
        if beacon.1 == row {
            beacons.insert(beacon.0);
        }
        let distance = find_distance(sensor, beacon);
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

fn find_distress_beacon(sensor_and_beacons: &SensorAndBeacons, max_coordinate: i64) -> Coordinates {
    let coverage_areas: Vec<(Coordinates, i64)> = sensor_and_beacons
        .iter()
        .map(|&(sensor, beacon)| (sensor, find_distance(&sensor, &beacon)))
        .collect();

    for j in 0..=max_coordinate {
        let mut i = 0;
        while i < max_coordinate {
            let current_location = (i, j);
            let mut covered = false;
            for &(sensor, radius) in &coverage_areas {
                let distance = find_distance(&sensor, &current_location);
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

fn solve_first_part(sensor_and_beacons: &SensorAndBeacons) -> usize {
    find_beacon_free_cells(sensor_and_beacons, 2000000)
}

fn solve_second_part(sensor_and_beacons: &SensorAndBeacons) -> i64 {
    let beacon = find_distress_beacon(sensor_and_beacons, 4000000);
    dbg!(&beacon);
    (beacon.0 * 4000000) + beacon.1
}

pub fn solve() -> (usize, i64) {
    let sensor_and_beacons = get_sensor_and_beacons(&utils::get_input("inputs/2022_15.txt"));
    (
        solve_first_part(&sensor_and_beacons),
        solve_second_part(&sensor_and_beacons),
    )
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
    fn should_get_sensor_and_beacons() {
        assert_eq!(
            get_sensor_and_beacons(EXAMPLE),
            vec![
                ((2, 18), (-2, 15)),
                ((9, 16), (10, 16)),
                ((13, 2), (15, 3)),
                ((12, 14), (10, 16)),
                ((10, 20), (10, 16)),
                ((14, 17), (10, 16)),
                ((8, 7), (2, 10)),
                ((2, 0), (2, 10)),
                ((0, 11), (2, 10)),
                ((20, 14), (25, 17)),
                ((17, 20), (21, 22)),
                ((16, 7), (15, 3)),
                ((14, 3), (15, 3)),
                ((20, 1), (15, 3)),
            ]
        );
    }

    #[test]
    fn should_solve_first_part() {
        let sensors_and_beacons = get_sensor_and_beacons(EXAMPLE);
        assert_eq!(find_beacon_free_cells(&sensors_and_beacons, 10), 26);
    }

    #[test]
    fn should_solve_second_part() {
        let sensors_and_beacons = get_sensor_and_beacons(EXAMPLE);
        assert_eq!(find_distress_beacon(&sensors_and_beacons, 20), (14, 11));
    }
}
