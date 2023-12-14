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
    let input = include_str!("../../inputs/day-14.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    let mut dish: Dish = input.parse()?;
    move_vertical(&mut dish, -1);

    Ok(calculate_score(&dish.grid))
}

fn part_2(input: &str) -> Result<usize> {
    let mut dish: Dish = input.parse()?;
    let cycles = 1_000_000_000;
    let mut seen = HashSet::new();
    let mut map = HashMap::<Vec<Vec<Tile>>, usize>::new();

    seen.insert(dish.grid.clone());
    for i in 0..cycles {
        run_a_cycle(&mut dish);
        if !seen.insert(dish.grid.clone()) {
            let last_seen = *map.get(&dish.grid).unwrap();
            let duration_of_a_cycle = i - last_seen;
            let remained_cycles = (cycles - last_seen) % duration_of_a_cycle;
            for _ in 0..remained_cycles {
                run_a_cycle(&mut dish);
            }
            return Ok(calculate_score(&dish.grid) - 1);
        }
        map.insert(dish.grid.clone(), i);
    }

    Ok(0)
}

fn run_a_cycle(dish: &mut Dish) {
    move_vertical(dish, -1);
    move_horizontal(dish, -1);
    move_vertical(dish, 1);
    move_horizontal(dish, 1);
}

fn move_horizontal(dish: &mut Dish, direction: i32) {
    if direction > 0 {
        for row in 0..dish.grid.len() {
            for col in (0..dish.grid[0].len()).rev() {
                if dish.grid[row][col].is_rounded() {
                    let mut idx = col;
                    while idx < dish.grid[0].len() - 1 {
                        if dish.grid[row][idx + 1] == Tile::Space {
                            idx += 1;
                        } else {
                            break;
                        }
                    }
                    if idx != col {
                        dish.grid[row][idx] = Tile::Rounded;
                        dish.grid[row][col] = Tile::Space;
                    }
                }
            }
        }
    } else {
        for row in 0..dish.grid.len() {
            for col in 0..dish.grid[0].len() {
                if dish.grid[row][col].is_rounded() {
                    let mut idx = col;
                    while idx >= 1 {
                        if dish.grid[row][idx - 1] == Tile::Space {
                            idx -= 1;
                        } else {
                            break;
                        }
                    }
                    if idx != col {
                        dish.grid[row][idx] = Tile::Rounded;
                        dish.grid[row][col] = Tile::Space;
                    }
                }
            }
        }
    }
}

fn move_vertical(dish: &mut Dish, direction: i32) {
    if direction < 0 {
        for row in 0..dish.grid.len() {
            for col in 0..dish.grid[0].len() {
                if dish.grid[row][col].is_rounded() {
                    let mut idx = row;
                    while idx >= 1 {
                        if dish.grid[idx - 1][col] == Tile::Space {
                            idx -= 1;
                        } else {
                            break;
                        }
                    }
                    if idx != row {
                        dish.grid[idx][col] = Tile::Rounded;
                        dish.grid[row][col] = Tile::Space;
                    }
                }
            }
        }
    } else {
        for row in (0..dish.grid.len()).rev() {
            for col in 0..dish.grid[0].len() {
                if dish.grid[row][col].is_rounded() {
                    let mut idx = row;
                    while idx < dish.grid.len() - 1 {
                        if dish.grid[idx + 1][col] == Tile::Space {
                            idx += 1;
                        } else {
                            break;
                        }
                    }
                    if idx != row {
                        dish.grid[idx][col] = Tile::Rounded;
                        dish.grid[row][col] = Tile::Space;
                    }
                }
            }
        }
    }
}

fn calculate_score(grid: &[Vec<Tile>]) -> usize {
    let mut counter = 0;
    for (y, row) in grid.iter().enumerate() {
        for tile in row.iter() {
            if tile.is_rounded() {
                counter += grid.len() - y;
            }
        }
    }
    counter
}

impl Display for Dish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                match self.grid[row][col] {
                    Tile::Squared => write!(f, "#")?,
                    Tile::Rounded => write!(f, "O")?,
                    Tile::Space => write!(f, ".")?,
                }
            }
            writeln!(f,)?;
        }

        Ok(())
    }
}
struct Dish {
    grid: Vec<Vec<Tile>>,
}

impl FromStr for Dish {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            grid: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => Tile::Squared,
                            'O' => Tile::Rounded,
                            '.' => Tile::Space,
                            _ => panic!("ERROR: Bad input {c}"),
                        })
                        .collect_vec()
                })
                .collect_vec(),
        })
    }
}

impl Tile {
    fn is_rounded(&self) -> bool {
        *self == Tile::Rounded
    }
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
enum Tile {
    Squared,
    Rounded,
    Space,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-14-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 136);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 64);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-14.txt")).unwrap(),
            108_857
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-14.txt")).unwrap(),
            95_274
        );
    }
}
