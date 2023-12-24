use aoc_2023_lib::main;
use itertools::Itertools;

use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-24.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<i32> {
    Ok(input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" @ ").unwrap();
            let left = left
                .split(", ")
                .map(|n| n.parse::<f64>().unwrap())
                .collect_vec();
            let x = left[0];
            let y = left[1];

            let right = right
                .split(", ")
                .map(|n| {
                    n.split_ascii_whitespace()
                        .next()
                        .unwrap()
                        .parse::<isize>()
                        .unwrap()
                })
                .collect_vec();

            let vx = right[0];
            let vy = right[1];

            let slope = (vy as f64 / vx as f64);

            (slope, y - x * slope, x, vx as f64)
        })
        .tuple_combinations()
        .filter_map(|(d1, d2)| {
            if d1.0 == d2.0 {
                return None;
            } else {
                let x = (d2.1 - d1.1) / (d1.0 - d2.0);
                let y = -(d2.0 * d1.1 - d2.1 * d1.0) / (d1.0 - d2.0);

                if 200_000_000_000_000f64 <= x
                    && x <= 400_000_000_000_000f64
                    && 200_000_000_000_000f64 <= y
                    && y <= 400_000_000_000_000f64
                {
                    if (d1.3 < 0f64 && x > d1.2)
                        || (d1.3 > 0f64 && x < d1.2)
                        || (d2.3 < 0f64 && x > d2.2)
                        || (d2.3 > 0f64 && x < d2.2)
                    {
                        return None;
                    }
                    return Some(1);
                } else {
                    None
                }
            }
        })
        .sum())
}

fn part_2(input: &str) -> Result<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-24-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 2);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 47);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-24.txt")).unwrap(),
            29_142
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../../inputs/day-24.txt")).unwrap(), 1);
    }
}
