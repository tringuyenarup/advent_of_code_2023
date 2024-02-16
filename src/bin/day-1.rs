use aoc_2023_lib::main;

use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-1.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(|line| {
            let results = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<_>>();
            match results.len() == 1 {
                true => calculate_number(
                    results.first().unwrap().to_digit(10).unwrap(),
                    results.first().unwrap().to_digit(10).unwrap(),
                ),
                false => calculate_number(
                    results.first().unwrap().to_digit(10).unwrap(),
                    results.last().unwrap().to_digit(10).unwrap(),
                ),
            }
        })
        .sum::<u32>())
}

fn part_2(input: &str) -> Result<u32> {
    let patterns = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let searcher = aho_corasick::AhoCorasick::builder()
        .build(patterns.clone())
        .unwrap();

    Ok(input
        .lines()
        .map(|line| {
            let results = searcher
                .find_overlapping_iter(line)
                .map(|mat| mat.pattern())
                .collect::<Vec<_>>();
            calculate_number(
                get_number(patterns[results.first().unwrap().as_usize()]),
                get_number(patterns[results.last().unwrap().as_usize()]),
            )
        })
        .sum::<u32>())
}

fn calculate_number(first_digit: u32, second_digit: u32) -> u32 {
    first_digit * 10 + second_digit
}

fn get_number(number: &str) -> u32 {
    match number {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("ERROR: Bad input"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-1-test-1.txt")).unwrap(),
            142
        );
        assert_eq!(
            part_2(include_str!("../../inputs/day-1-test-2.txt")).unwrap(),
            281
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-1.txt")).unwrap(),
            54_632
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-1.txt")).unwrap(),
            54_019
        );
    }
}
