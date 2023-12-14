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
            if results.len() == 1 {
                results[0].to_digit(10).unwrap() * 11
            } else {
                results[0].to_digit(10).unwrap() * 10
                    + results.last().unwrap().to_digit(10).unwrap()
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
            get_number(patterns[results.first().unwrap().as_usize()]) * 10
                + get_number(patterns[results.last().unwrap().as_usize()])
        })
        .sum::<u32>())
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
