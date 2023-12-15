use aoc_2023_lib::main;

use std::{collections::VecDeque, error::Error};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-15.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<i32> {
    Ok(input.split(',').map(hash).sum())
}

fn part_2(input: &str) -> Result<usize> {
    Ok(input
        .split(',')
        .fold(
            vec![VecDeque::default(); 256],
            |mut acc: Vec<VecDeque<(&str, u8)>>, len| {
                let (label, focal_length) = match len.contains('=') {
                    true => {
                        let (a, b) = len.split_once('=').unwrap();
                        (a, b.parse::<u8>().unwrap())
                    }
                    false => (len.trim_end_matches('-'), 0),
                };
                let box_index = hash(label);

                if focal_length == 0 {
                    if let Some(pos) = acc[box_index as usize]
                        .iter()
                        .position(|&(l, _)| l == label)
                    {
                        acc[box_index as usize].remove(pos);
                    }
                } else if let Some(pos) = acc[box_index as usize]
                    .iter()
                    .position(|&(l, _)| l == label)
                {
                    acc[box_index as usize][pos] = (label, focal_length);
                } else {
                    acc[box_index as usize].push_back((label, focal_length));
                }
                acc
            },
        )
        .iter()
        .enumerate()
        .filter_map(|(index, b)| {
            if !b.is_empty() {
                Some(
                    (index + 1)
                        * b.iter()
                            .enumerate()
                            .map(|(i, v)| (i + 1) * (v.1 as usize))
                            .sum::<usize>(),
                )
            } else {
                None
            }
        })
        .sum::<usize>())
}

fn hash(data: &str) -> i32 {
    data.chars().fold(0, |acc, c| {
        let number = (c as u8) as i32;
        ((acc + number) * 17) % 256
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-15-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 1_320);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 145);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-15.txt")).unwrap(),
            504_036
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-15.txt")).unwrap(),
            295_719
        );
    }
}
