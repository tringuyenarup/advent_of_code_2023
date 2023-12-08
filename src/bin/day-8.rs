use aoc_2023_lib::{err, main, utils::lcm};
use itertools::Itertools;

use std::{collections::HashMap, error::Error, str::FromStr};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-8.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}
#[allow(clippy::explicit_counter_loop)]
fn part_1(input: &str) -> Result<i32> {
    let (direction, network) = input.split_once("\n\n").unwrap();
    let network: Network = network.parse()?;
    let moves: Vec<char> = direction.chars().collect_vec();

    let mut steps = 0;
    let mut current_node = "AAA";
    loop {
        for next_step in moves.iter().cycle() {
            if current_node == "ZZZ" {
                return Ok(steps);
            }
            steps += 1;
            let next_move = match *next_step {
                'L' => &network.nodes[current_node].0,
                'R' => &network.nodes[current_node].1,
                _ => return err!("bad move"),
            };
            current_node = next_move;
        }
    }
}
#[allow(clippy::explicit_counter_loop)]
fn find_destination(network: &Network, moves: &[char], start: &str) -> usize {
    let mut steps = 0;
    let mut current_node = start;
    loop {
        for next_step in moves.iter().cycle() {
            if current_node.ends_with('Z') {
                return steps;
            }
            steps += 1;
            let next_move = match *next_step {
                'L' => &network.nodes[current_node].0,
                'R' => &network.nodes[current_node].1,
                _ => panic!("Bad move"),
            };
            current_node = next_move;
        }
    }
}

fn part_2(input: &str) -> Result<usize> {
    let (direction, network) = input.split_once("\n\n").unwrap();
    let network: Network = network.parse()?;
    let moves: Vec<char> = direction.chars().collect_vec();

    let current_nodes = network
        .nodes
        .keys()
        .filter(|&k| k.ends_with('A'))
        .collect_vec();

    let steps = current_nodes
        .iter()
        .map(|&start| find_destination(&network, &moves, start))
        .collect_vec();

    Ok(steps.into_iter().fold(1, |mut acc, step| {
        acc = lcm(acc, step);
        acc
    }))
}
#[derive(Debug)]
struct Network {
    nodes: HashMap<String, (String, String)>,
}

impl FromStr for Network {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        Ok(Network {
            nodes: input
                .lines()
                .map(|line| {
                    let (from, to) = line.split_once(" = ").unwrap();
                    let (left, right) = to.split_once(", ").unwrap();
                    (
                        String::from(from),
                        (
                            String::from(left.trim_start_matches('(')),
                            String::from(right.trim_end_matches(')')),
                        ),
                    )
                })
                .collect::<HashMap<String, (String, String)>>(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-8-test-1.txt")).unwrap(),
            2
        );
        assert_eq!(
            part_1(include_str!("../../inputs/day-8-test-2.txt")).unwrap(),
            6
        );
        assert_eq!(
            part_2(include_str!("../../inputs/day-8-test-3.txt")).unwrap(),
            6
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-8.txt")).unwrap(),
            11_911
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-8.txt")).unwrap(),
            10_151_663_816_849
        );
    }
}
