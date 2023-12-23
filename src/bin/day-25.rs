use aoc_2023_lib::main;

use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-25.txt");
    (part_1(input).unwrap(),part_1(input).unwrap())
}

fn part_1(input: &str) -> Result<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-25-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 1);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../../inputs/day-25.txt")).unwrap(), 1);
    }
}
