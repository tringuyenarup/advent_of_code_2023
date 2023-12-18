use aoc_2023_lib::main;
use itertools::Itertools;

use std::{error::Error, str::FromStr};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-18.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<i32> {
    let plan = input.parse::<Plan>()?;
    let (mut r, mut c, mut point) = (0, 0, 0);

    let mut points = vec![(0, 0)];

    for (direction, offset, _) in plan.digs {
        match direction {
            'U' => {
                r -= offset;
            }
            'D' => {
                r += offset;
            }
            'L' => {
                c -= offset;
            }
            'R' => {
                c += offset;
            }
            _ => panic!("ERROR: bad direction {direction}"),
        }
        points.push((r, c));
        point += offset;
    }
    let a = points
        .iter()
        .tuple_windows()
        .map(|(p1, p2)| (p1.1 + p2.1) * (p2.0 - p1.0))
        .sum::<i32>();
    Ok((point + a) / 2 + 1)
}

fn part_2(input: &str) -> Result<isize> {
    let plan = input.parse::<Plan>()?;
    let (mut r, mut c, mut point) = (0, 0, 0);

    let mut points = vec![(0, 0)];

    for (_, _, hex) in plan.digs {
        let (offset, direction) = (
            isize::from_str_radix(&hex[1..6], 16).unwrap(),
            hex[6..].parse::<u8>().unwrap(),
        );
        match direction {
            3 => {
                r -= offset;
            }
            1 => {
                r += offset;
            }
            2 => {
                c -= offset;
            }
            0 => {
                c += offset;
            }
            _ => panic!("ERROR: bad direction {direction}"),
        }
        points.push((r, c));
        point += offset;
    }
    let a = points
        .iter()
        .tuple_windows()
        .map(|(p1, p2)| (p1.1 + p2.1) * (p2.0 - p1.0))
        .sum::<isize>();
    Ok((point + a) / 2 + 1)
}

struct Plan {
    digs: Vec<(char, i32, String)>,
}

impl FromStr for Plan {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        Ok(Self {
            digs: input
                .lines()
                .map(|line| {
                    let mut line = line.split_ascii_whitespace();
                    let direction = line
                        .next()
                        .expect("ERROR: input should have move direction");
                    let offset = line
                        .next()
                        .expect("ERROR: input should have offset for move")
                        .parse::<i32>()
                        .unwrap();
                    let color = line
                        .next()
                        .expect("ERROR: input should have color for a dig")
                        .trim_start_matches("(")
                        .trim_end_matches(")");

                    (
                        direction.chars().next().unwrap(),
                        offset,
                        String::from(color),
                    )
                })
                .collect_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-18-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 62);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 952_408_144_115);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-18.txt")).unwrap(),
            52_035
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../../inputs/day-18.txt")).unwrap(), 1);
    }
}
