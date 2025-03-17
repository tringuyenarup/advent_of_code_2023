use aoc_2023_lib::main;

use std::{error::Error, str::FromStr};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-6.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<u32> {
    let records: Records = input.parse().expect("parse records");

    Ok(records
        .times
        .iter()
        .zip(records.distances.iter())
        .map(|(time, distance)| {
            let mut counter = 0;
            for t in 0..=*time {
                if t * (*time - t) > *distance {
                    counter += 1;
                }
            }
            counter
        })
        .product::<u32>())
}

fn part_2(input: &str) -> Result<u64> {
    let (t, d) = input.split_once('\n').unwrap();
    let time = t
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = d
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let delta = (time.pow(2) - (4 * distance)) as f64;

    // x2 < t < x1
    let x1 = ((time as f64) * -1_f64 - delta.sqrt()) / (-2_f64);
    let x2 = ((time as f64) * -1_f64 + delta.sqrt()) / (-2_f64);

    Ok((x1.floor() - x2.ceil()) as u64 + 1)
}

#[derive(Debug)]
struct Records {
    times: Vec<u32>,
    distances: Vec<u32>,
}

impl FromStr for Records {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let (time, distances) = input.split_once('\n').unwrap();
        Ok(Self {
            times: time
                .split_ascii_whitespace()
                .skip(1)
                .map(|t| t.parse().unwrap())
                .collect(),
            distances: distances
                .split_ascii_whitespace()
                .skip(1)
                .map(|t| t.parse().unwrap())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-6-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 288);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 71_503);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-6.txt")).unwrap(),
            781_200
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-6.txt")).unwrap(),
            49_240_091
        );
    }
}
