use aoc_2023_lib::main;
use itertools::Itertools;

use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-9.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<isize> {
    Ok(input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split_ascii_whitespace()
                .map(|number| {
                    number
                        .parse::<isize>()
                        .unwrap_or_else(|_| panic!("bad input {number}"))
                })
                .collect_vec();
            let mut all_stages = Vec::new();
            all_stages.push(numbers.clone());
            loop {
                let new_numbers = numbers
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec();
                if new_numbers.iter().all(|n| *n == 0) {
                    return all_stages.iter().fold(0, |mut acc: isize, n| {
                        acc += n.last().unwrap();
                        acc
                    });
                } else {
                    all_stages.push(new_numbers.clone());
                    numbers = new_numbers;
                }
            }
        })
        .sum::<isize>())
}

fn part_2(input: &str) -> Result<isize> {
    Ok(input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split_ascii_whitespace()
                .map(|number| {
                    number
                        .parse::<isize>()
                        .unwrap_or_else(|_| panic!("bad input {number}"))
                })
                .collect_vec();
            let mut all_stages = Vec::new();
            all_stages.push(numbers.clone());
            loop {
                let new_numbers = numbers
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec();
                if new_numbers.iter().all(|n| *n == 0) {
                    all_stages.reverse();
                    return all_stages.iter().fold(0, |mut acc: isize, n| {
                        acc = n.first().unwrap() - acc;
                        acc
                    });
                } else {
                    all_stages.push(new_numbers.clone());
                    numbers = new_numbers;
                }
            }
        })
        .sum::<isize>())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-9-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 114);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 2);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-9.txt")).unwrap(),
            1_681_758_908
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../../inputs/day-9.txt")).unwrap(), 803);
    }
}
