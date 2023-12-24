use aoc_2023_lib::main;

use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-17.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<u16> {
    todo!()
}

fn part_2(input: &str) -> Result<u16> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-17-test.txt");

    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 102);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 94);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-17.txt")).unwrap(),
            902
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-17.txt")).unwrap(),
            1_073
        );
    }
}
