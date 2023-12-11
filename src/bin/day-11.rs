use aoc_2023_lib::main;
use itertools::Itertools;

use std::{collections::HashSet, error::Error, str::FromStr};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-11.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    expand(input, 2)
}

fn part_2(input: &str) -> Result<usize> {
    expand(input, 1_000_000)
}

fn expand(input: &str, factor: usize) -> Result<usize> {
    let image: Image = input.parse()?;
    let tranpose_grid = transpose(image.grid.clone());

    Ok(image
        .galaxies
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| {
            let mut distance = ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as usize;
            let expand_factor = factor - 1;
            let (from_row, to_row) = (std::cmp::min(p1.0, p2.0), std::cmp::max(p1.0, p2.0));
            let (from_col, to_col) = (std::cmp::min(p1.1, p2.1), std::cmp::max(p1.1, p2.1));
            for row in from_row + 1..to_row {
                if image.grid[row as usize].iter().all(|tile| tile.is_space()) {
                    distance += expand_factor;
                }
            }
            for col in from_col + 1..to_col {
                if tranpose_grid[col as usize]
                    .iter()
                    .all(|tile| tile.is_space())
                {
                    distance += expand_factor;
                }
            }
            distance
        })
        .sum::<usize>())
}

#[derive(Clone)]
struct Image {
    grid: Vec<Vec<Tile>>,
    galaxies: HashSet<(i32, i32)>,
}

impl FromStr for Image {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let mut galaxies = HashSet::new();
        let grid = input
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, ch)| match ch {
                        '.' => Tile::Space,
                        '#' => {
                            galaxies.insert((y as i32, x as i32));
                            Tile::Galaxy
                        }
                        _ => panic!("bad input"),
                    })
                    .collect_vec()
            })
            .collect_vec();

        Ok(Self { grid, galaxies })
    }
}
#[derive(Clone, PartialEq)]
enum Tile {
    Galaxy,
    Space,
}
impl Tile {
    fn is_space(&self) -> bool {
        *self == Tile::Space
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-11-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 374);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 1_030);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-11.txt")).unwrap(),
            9_769_724
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-11.txt")).unwrap(),
            603020563700
        );
    }
}
