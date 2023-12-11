use aoc_2023_lib::main;
use itertools::Itertools;

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
    str::FromStr,
    vec,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-10.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    let maze: Maze = input.parse()?;
    let visited = find_loop(&maze)?;
    Ok(visited.len() / 2)
}

fn part_2(input: &str) -> Result<i32> {
    let maze: Maze = input.parse()?;
    let visited = find_loop(&maze)?;

    let mut count = 0;
    let mut inside = false;

    for row in 0..maze.tiles.len() {
        let mut tile = Tile::Ground;
        for col in 0..maze.tiles[0].len() {
            if visited.contains(&(row as i32, col as i32)) {
                let ch = maze.tiles[row][col];
                match ch {
                    Tile::Vertical => inside = !inside,
                    Tile::NorthEast | Tile::SouthEast => tile = ch,
                    Tile::SouthWest => {
                        if tile == Tile::NorthEast {
                            inside = !inside;
                        }
                    }
                    Tile::NorthWest => {
                        if tile == Tile::SouthEast {
                            inside = !inside;
                        }
                    }
                    _ => {}
                }
            } else if inside {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn find_loop(maze: &Maze) -> Result<HashSet<(i32, i32)>> {
    let mut distances = HashMap::new();
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    let mut distance = 0;
    let start = maze.get_start_point()?;

    distances.insert(start, distance);
    visited.insert(start);
    queue.push(
        find_neighbours(&start.0, &start.1, maze, &mut visited)
            .unwrap_or_else(|| panic!("ERROR: there is no neighbours of the start")),
    );

    while let Some(next_nodes) = queue.pop() {
        let mut next = Vec::new();
        distance += 1;
        for (row, col) in next_nodes {
            let next_nodes = find_neighbours(&row, &col, maze, &mut visited);
            distances.insert((row, col), distance);

            if let Some(next_nodes) = next_nodes {
                next.extend(next_nodes);
            }
        }
        if !next.is_empty() {
            queue.push(next)
        }
    }
    Ok(visited)
}

fn find_neighbours(
    row: &i32,
    col: &i32,
    maze: &Maze,
    visited: &mut HashSet<(i32, i32)>,
) -> Option<Vec<(i32, i32)>> {
    let directions = match maze.tiles[*row as usize][*col as usize] {
        Tile::Vertical => Some(vec![(1, 0), (-1, 0)]),
        Tile::Horizontal => Some(vec![(0, -1), (0, 1)]),
        Tile::NorthWest => Some(vec![(-1, 0), (0, -1)]),
        Tile::NorthEast => Some(vec![(-1, 0), (0, 1)]),
        Tile::SouthWest => Some(vec![(1, 0), (0, -1)]),
        Tile::SouthEast => Some(vec![(1, 0), (0, 1)]),
        Tile::Start => Some(vec![(1, 0), (-1, 0), (0, 1), (0, -1)]),
        _ => None,
    };
    directions.map(|directions| {
        directions
            .iter()
            .filter_map(|&(dr, dc)| {
                let (new_row, new_col) = (row + dr, col + dc);
                if 0 <= new_row
                    && new_row < maze.tiles.len() as i32
                    && 0 <= new_col
                    && new_col < maze.tiles[0].len() as i32
                    && maze.tiles[new_row as usize][new_col as usize] != Tile::Ground
                    && visited.insert((new_row, new_col))
                {
                    Some((new_row, new_col))
                } else {
                    None
                }
            })
            .collect_vec()
    })
}

struct Maze {
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    fn get_start_point(&self) -> Result<(i32, i32)> {
        let (y, row) = self
            .tiles
            .iter()
            .enumerate()
            .find(|&(_, row)| row.contains(&Tile::Start))
            .unwrap_or_else(|| panic!("ERROR: Expect to find a starting y"));
        let (x, _) = row
            .iter()
            .enumerate()
            .find(|&(_, tile)| *tile == Tile::Start)
            .unwrap_or_else(|| panic!("ERROR: Expect to find a starting x"));
        Ok((y as i32, x as i32))
    }
}
#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                match *tile {
                    Tile::Vertical => write!(f, "|")?,
                    Tile::Horizontal => write!(f, "-")?,
                    Tile::NorthWest => write!(f, "J")?,
                    Tile::NorthEast => write!(f, "L")?,
                    Tile::SouthWest => write!(f, "7")?,
                    Tile::SouthEast => write!(f, "F")?,
                    Tile::Ground => write!(f, ".")?,
                    Tile::Start => write!(f, "S")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for Maze {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let tiles = input
            .lines()
            .enumerate()
            .map(|(_row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(_col, tile)| match tile {
                        '|' => Tile::Vertical,
                        '-' => Tile::Horizontal,
                        'L' => Tile::NorthEast,
                        'J' => Tile::NorthWest,
                        '7' => Tile::SouthWest,
                        'F' => Tile::SouthEast,
                        '.' => Tile::Ground,
                        'S' => Tile::Start,
                        _ => {
                            panic!("bad input");
                        }
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();

        Ok(Self { tiles })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-10-test-1.txt")).unwrap(),
            8
        );
        assert_eq!(
            part_1(include_str!("../../inputs/day-10-test-2.txt")).unwrap(),
            4
        );
        assert_eq!(
            part_2(include_str!("../../inputs/day-10-test-3.txt")).unwrap(),
            4
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-10.txt")).unwrap(),
            6_754
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-10.txt")).unwrap(),
            567
        );
    }
}
