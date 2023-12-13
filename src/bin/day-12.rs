use aoc_2023_lib::main;
use itertools::Itertools;

use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day12/src/main.rs
main! {
    let input = include_str!("../../inputs/day-12.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    Ok(arrangements(input, 1))
}

fn part_2(input: &str) -> Result<usize> {
    Ok(arrangements(input, 5))
}

fn arrangements(input: &str, repetitions: usize) -> usize {
    input
        .lines()
        .map(|line| {
            let (springs_single_rep, groups_single_rep) = parse_spring(line);

            let mut springs = Vec::<Spring>::new();
            let mut groups = Vec::<usize>::new();

            let mut stack = Vec::new();
            let mut cache = Vec::new();

            springs.reserve((springs_single_rep.len() + repetitions) - 1);
            groups.reserve(groups_single_rep.len() * repetitions);

            for _ in 0..repetitions {
                if !springs.is_empty() {
                    springs.push(Spring::Unknown);
                }
                springs.extend(&springs_single_rep);
                groups.extend(&groups_single_rep);
            }

            cache.resize((groups.len() - 1) * springs.len(), None);
            stack.reserve(groups.len() - 1);

            let mut count = 0;
            let mut pos = 0;

            loop {
                let len = groups[stack.len()];
                let end = pos + len;

                // There's a broken spring that's not included in a group, or we've gone past the end
                if end > springs.len() || (pos > 0 && springs[pos - 1] == Spring::Broken) {
                    if let Some((x, y)) = stack.pop() {
                        pos = x;
                        cache[stack.len() * springs.len() + pos] = Some(count);
                        count += y;
                        pos += 1;
                        continue;
                    } else {
                        break;
                    }
                }

                // Not a valid position
                if (end < springs.len() && springs[end] == Spring::Broken)
                    || springs[pos..end].iter().any(|&x| x == Spring::Operational)
                {
                    pos += 1;
                    continue;
                }
                // number of remain groups = stack group
                if stack.len() == groups.len() - 1 {
                    if springs[end..].iter().all(|&x| x != Spring::Broken) {
                        count += 1;
                    }
                    pos += 1;
                } else if let Some(old) = cache[stack.len() * springs.len() + pos] {
                    count += old;
                    pos += 1;
                } else {
                    stack.push((pos, count));
                    count = 0;
                    pos = end + 1;
                }
            }

            count
        })
        .sum()
}

fn parse_spring(line: &str) -> (Vec<Spring>, Vec<usize>) {
    let mut parts = line.split_ascii_whitespace();
    (
        parts
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '#' => Spring::Broken,
                '.' => Spring::Operational,
                '?' => Spring::Unknown,
                _ => panic!(),
            })
            .collect_vec(),
        parts
            .next()
            .unwrap()
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect_vec(),
    )
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Spring {
    Unknown,
    Broken,
    Operational,
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
