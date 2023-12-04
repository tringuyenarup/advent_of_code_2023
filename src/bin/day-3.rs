use aoc_2023_lib::main;

use std::{collections::HashMap, error::Error, str::FromStr};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-3.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<u32> {
    let scheme: Scheme = input.parse()?;
    Ok(scheme.symbols.into_iter().flat_map(|(_, v)| v).sum::<u32>())
}

fn part_2(input: &str) -> Result<u32> {
    let scheme: Scheme = input.parse()?;
    Ok(scheme
        .symbols
        .into_iter()
        .filter_map(|((_, _, symbol), v)| {
            if symbol == '*' && v.len() == 2 {
                Some(v.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum::<u32>())
}
#[derive(Debug)]
struct Scheme {
    //  map contains all symbols position, type and their neighbours
    symbols: HashMap<(usize, usize, char), Vec<u32>>,
}

impl FromStr for Scheme {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let input = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        // 1. check if the character is symbols
        // 2. if the neighbor is a number, put them in the vec
        let mut symbols = HashMap::new();
        for (y, row) in input.iter().enumerate() {
            let mut x = 0;
            while x < row.len() {
                let (start, mut symbol) = (x, None);
                while x < row.len() && row[x].is_ascii_digit() {
                    symbol = symbol.or_else(|| find_symbol(&input, y, x));
                    x += 1;
                }
                if let Some(symbol) = symbol {
                    let number = row[start..x]
                        .iter()
                        .fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap());
                    symbols.entry(symbol).or_insert(Vec::new()).push(number);
                }
                x += 1;
            }
        }

        Ok(Scheme { symbols })
    }
}

fn find_symbol(input: &[Vec<char>], row: usize, col: usize) -> Option<(usize, usize, char)> {
    for (dr, dc) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let (new_r, new_c) = ((dr + row as i32) as usize, (dc + col as i32) as usize);
        let Some(&s) = input.get(new_r).and_then(|line| line.get(new_c)) else {
            continue;
        };
        if s != '.' && !s.is_ascii_digit() {
            return Some((new_r, new_c, s));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-3-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 4_361);
        // assert_eq!(part_2(TEST_INPUT).unwrap(), 467_835);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-3.txt")).unwrap(),
            527_369
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-3.txt")).unwrap(),
            73_074_886
        );
    }
}
