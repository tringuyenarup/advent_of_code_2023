use aoc_2023_lib::main;
use itertools::Itertools;

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
    str::FromStr,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-21.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    let garden = input.parse::<Map>()?;

    Ok(garden.step(64))
}

fn part_2(input: &str) -> Result<usize> {
    let garden = input.parse::<Map>()?;

    Ok(garden.step_repeat(50))
}
struct Map {
    start: (usize, usize),
    tiles: Vec<Vec<Tile>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.iter() {
            for cell in row.iter() {
                match *cell {
                    Tile::Garden => write!(f, ".")?,
                    Tile::Rock => write!(f, "#")?,
                }
            }
            writeln!(f,)?
        }

        Ok(())
    }
}

impl Map {
    fn step_repeat(&self, steps: isize) -> usize {
        let mut queue = Vec::new();
        queue.push(self.start);
        let mut counter = 0;
        let mut all_points = HashMap::new();
        while counter < steps {
            let mut new_queue: HashMap<(usize, usize), usize> = HashMap::new();
            while let Some(pos) = queue.pop() {
                let new_pos = vec![
                    ((pos.0 + 1) % self.tiles.len(), pos.1),
                    ((pos.0 + self.tiles.len() - 1) % self.tiles.len(), pos.1),
                    (pos.0, (pos.1 + 1) % self.tiles[0].len()),
                    (
                        pos.0,
                        (pos.1 + self.tiles[0].len() - 1) % self.tiles[0].len(),
                    ),
                ]
                .iter()
                .filter_map(|&pos| {
                    if self.tiles[pos.0][pos.1].is_garden() {
                        Some(pos)
                    } else {
                        None
                    }
                })
                .fold(HashMap::new(), |mut acc, p| {
                    *acc.entry(p).or_insert(0) += 1;
                    acc
                });
                for (k, v) in new_pos.clone() {
                    *new_queue.entry(k).or_default() += v;
                }
            }
            for (k, v) in new_queue.clone() {
                *all_points.entry(k).or_insert(1) *= v;
            }
            queue = new_queue.keys().map(|k| k.clone()).collect();

            counter += 1;
        }
        all_points.values().sum()
    }

    fn step(&self, steps: isize) -> usize {
        let mut queue = Vec::new();
        queue.push(self.start);
        let mut counter = 0;

        while counter < steps {
            let mut new_queue = HashSet::new();
            while let Some(pos) = queue.pop() {
                let new_pos = [
                    (pos.0 + 1, pos.1),
                    (pos.0.wrapping_sub(1), pos.1),
                    (pos.0, pos.1 + 1),
                    (pos.0, pos.1.wrapping_sub(1)),
                ]
                .iter()
                .filter_map(|(new_r, new_c)| {
                    if *new_r > self.tiles.len() - 1 || *new_c > self.tiles[0].len() - 1 {
                        None
                    } else {
                        if self.tiles[*new_r][*new_c].is_garden() {
                            Some((*new_r, *new_c))
                        } else {
                            None
                        }
                    }
                })
                .collect_vec();
                new_queue.extend(new_pos);
            }
            queue = new_queue.iter().cloned().collect_vec();
            counter += 1;
        }
        queue.len()
    }
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let mut start = (0, 0);
        let tiles = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        '#' => Tile::Rock,
                        '.' => Tile::Garden,
                        'S' => {
                            start = (row, col);
                            Tile::Garden
                        }
                        _ => panic!("ERROR: BAD INPUT"),
                    })
                    .collect_vec()
            })
            .collect_vec();
        Ok(Self { start, tiles })
    }
}
#[derive(PartialEq, Clone)]
enum Tile {
    Garden,
    Rock,
}
impl Tile {
    fn is_garden(&self) -> bool {
        *self == Tile::Garden
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-21-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 42);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 1);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-21.txt")).unwrap(),
            3_503
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../../inputs/day-21.txt")).unwrap(), 1);
    }
}
