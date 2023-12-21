use aoc_2023_lib::main;
use itertools::Itertools;

use std::{collections::HashMap, error::Error, str::FromStr};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-12.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            let mut spring = line.parse::<Spring>().unwrap();
            spring.score(0)
        })
        .sum())
}

fn part_2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            let mut spring = line.parse::<Spring>().unwrap();
            spring.score(5)
        })
        .sum())
}

struct Spring {
    pattern: Vec<char>,
    sizes: Vec<usize>,
}

impl Spring {
    fn score(&mut self, scale_factor: usize) -> usize {
        let mut cache = HashMap::new();
        if scale_factor != 0 {
            self.scale(scale_factor);
        }
        Self::arrange(&self.pattern, &self.sizes, &mut cache)
    }

    fn arrange(
        pattern: &[char],
        sizes: &[usize],
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(arrangements) = cache.get(&(pattern.len(), sizes.len())) {
            return *arrangements;
        }
        if sizes.is_empty() {
            return (!pattern.contains(&'#')) as usize;
        }

        let min_remaining = sizes.iter().sum::<usize>() + sizes.len() - 1;

        if pattern.len() < min_remaining {
            return 0;
        }

        let result = match pattern[0] {
            '.' => Self::arrange(&pattern[1..], sizes, cache),
            '#' => Self::arrange_hash(pattern, sizes, cache),
            '?' => {
                Self::arrange(&pattern[1..], sizes, cache)
                    + Self::arrange_hash(pattern, sizes, cache)
            }
            _ => panic!("ERROR: bad input"),
        };
        cache.insert((pattern.len(), sizes.len()), result);
        result
    }
    fn arrange_hash(
        pattern: &[char],
        sizes: &[usize],
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if pattern.len() < sizes[0] || pattern[0..sizes[0]].contains(&'.') {
            return 0;
        }
        if pattern.len() == sizes[0] {
            return (sizes.len() == 1) as usize;
        }
        if pattern[sizes[0]] == '#' {
            return 0;
        }

        Self::arrange(&pattern[sizes[0] + 1..], &sizes[1..], cache)
    }
    fn scale(&mut self, scale_factor: usize) {
        let mut pattern: Vec<char> = Vec::new();
        for _ in 0..scale_factor - 1 {
            pattern.extend(self.pattern.iter().chain([&'?']));
        }
        let mut sizes: Vec<usize> = Vec::new();
        pattern.extend(self.pattern.iter());
        for _ in 0..scale_factor {
            sizes.extend(self.sizes.iter());
        }
        self.pattern = pattern;
        self.sizes = sizes;
    }
}

impl FromStr for Spring {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (pattern, nums) = s.split_once(' ').unwrap();
        let pattern = pattern.chars().collect_vec();
        let sizes = nums
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();
        Ok(Self { pattern, sizes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-12-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 21);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 525_152);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-12.txt")).unwrap(),
            7_843
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-12.txt")).unwrap(),
            10_153_896_718_999
        );
    }
}
