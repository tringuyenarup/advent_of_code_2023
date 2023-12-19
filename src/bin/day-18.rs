use aoc_2023_lib::main;

use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-18.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<isize> {
    let dig_plans = parse(input, false);
    shoelace_algorithm(dig_plans)
}

fn part_2(input: &str) -> Result<isize> {
    let dig_plans = parse(input, true);
    shoelace_algorithm(dig_plans)
}

fn shoelace_algorithm(dig_plans: impl Iterator<Item = (char, isize)>) -> Result<isize> {
    let mut start = (0, 0);
    let mut area = 0;
    for (direction, offset) in dig_plans {
        let new_start = match direction {
            'U' | '3' => (start.0 - offset, start.1),
            'D' | '1' => (start.0 + offset, start.1),
            'L' | '2' => (start.0, start.1 - offset),
            'R' | '0' => (start.0, start.1 + offset),
            _ => panic!("ERROR: bad input {direction}"),
        };
        area += (new_start.0 - start.0) * (start.1 + new_start.1) + offset;
        start = new_start;
    }

    Ok(area / 2 + 1)
}

fn parse(input: &str, hex_mode: bool) -> impl Iterator<Item = (char, isize)> + '_ {
    input.lines().map(move |line| {
        let mut line = line.split_ascii_whitespace();
        let direction = line
            .next()
            .expect("ERROR: input should have move direction");
        let offset = line
            .next()
            .expect("ERROR: input should have offset for move")
            .parse::<isize>()
            .unwrap();
        let hex_data = line
            .next()
            .expect("ERROR: input should have color for a dig")
            .trim_start_matches('(')
            .trim_end_matches(')');

        if hex_mode {
            (
                hex_data.chars().last().unwrap(),
                isize::from_str_radix(&hex_data[1..6], 16).expect("ERROR: bad input"),
            )
        } else {
            (direction.chars().next().unwrap(), offset)
        }
    })
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
        assert_eq!(
            part_2(include_str!("../../inputs/day-18.txt")).unwrap(),
            60_612_092_439_765
        );
    }
}
