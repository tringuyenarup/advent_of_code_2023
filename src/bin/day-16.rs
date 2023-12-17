use aoc_2023_lib::main;

use std::{borrow::BorrowMut, error::Error, str::FromStr};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-16.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<u16> {
    let mut contraption = input.parse::<Contraption>()?;

    Ok(contraption.count_energy((BeamDir::Right, 0, 0)))
}

fn part_2(input: &str) -> Result<u16> {
    let mut contraption = input.parse::<Contraption>()?;

    let height = contraption.grid.len();
    let width = contraption.grid[0].len();

    Ok((0..height)
        .flat_map(|row| [(BeamDir::Right, row, 0), (BeamDir::Left, row, width - 1)].into_iter())
        .chain(
            (0..width).flat_map(|col| {
                [(BeamDir::Down, 0, col), (BeamDir::Up, height - 1, col)].into_iter()
            }),
        )
        .map(|start| {
            // reset the map
            contraption.reset(&height, &width);
            // count again for it
            contraption.count_energy(start)
        })
        .max()
        .unwrap())
}

struct Contraption {
    grid: Vec<Vec<(Tile, u8)>>,
}

impl Contraption {
    fn reset(&mut self, height: &usize, width: &usize) {
        for r in 0..*height {
            for c in 0..*width {
                self.grid[r][c].1 = 0;
            }
        }
    }
    fn count_energy(&mut self, start_beam: (BeamDir, usize, usize)) -> u16 {
        let mut counter = 0;
        let mut beams = vec![start_beam];

        while let Some((current_direction, row, col)) = beams.pop() {
            let (tile, is_visited) = self.grid[row][col].borrow_mut();
            // if the tile is already visited
            if *is_visited & current_direction as u8 != 0 {
                continue;
            }
            if *is_visited == 0 {
                counter += 1;
            }

            *is_visited |= current_direction as u8;

            let new_beams_directions = find_new_directions(tile, &current_direction);
            for &new_direction in &new_beams_directions {
                let (new_row, new_col) = match new_direction {
                    BeamDir::Right => (row, col + 1),
                    BeamDir::Down => (row + 1, col),
                    BeamDir::Left => (row, col.wrapping_sub(1)),
                    BeamDir::Up => (row.wrapping_sub(1), col),
                };

                if new_row >= self.grid.len() || new_col >= self.grid[new_row].len() {
                    continue;
                }
                beams.push((new_direction, new_row, new_col));
            }
        }
        counter
    }
}

fn find_new_directions(tile: &Tile, current_direction: &BeamDir) -> Vec<BeamDir> {
    let mut new_beams_directions = Vec::new();
    match *tile {
        Tile::Space => new_beams_directions.push(*current_direction),
        Tile::MirrorUR => new_beams_directions.push(match current_direction {
            BeamDir::Right => BeamDir::Down,
            BeamDir::Down => BeamDir::Right,
            BeamDir::Left => BeamDir::Up,
            BeamDir::Up => BeamDir::Left,
        }),

        Tile::MirrorUL => new_beams_directions.push(match current_direction {
            BeamDir::Right => BeamDir::Up,
            BeamDir::Down => BeamDir::Left,
            BeamDir::Left => BeamDir::Down,
            BeamDir::Up => BeamDir::Right,
        }),
        Tile::SplitterVert => {
            if *current_direction as u8 & 0b1010 != 0 {
                new_beams_directions.push(*current_direction);
            } else {
                new_beams_directions.extend(&[BeamDir::Up, BeamDir::Down]);
            }
        }

        Tile::SplitterHoriz => {
            if *current_direction as u8 & 0b0101 != 0 {
                new_beams_directions.push(*current_direction);
            } else {
                new_beams_directions.extend(&[BeamDir::Left, BeamDir::Right]);
            }
        }
    };
    new_beams_directions
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BeamDir {
    Right = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Up = 0b1000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    MirrorUR,
    MirrorUL,
    SplitterVert,
    SplitterHoriz,
    Space,
}

impl FromStr for Contraption {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        Ok(Self {
            grid: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            (
                                match c {
                                    '\\' => Tile::MirrorUR,
                                    '/' => Tile::MirrorUL,
                                    '|' => Tile::SplitterVert,
                                    '-' => Tile::SplitterHoriz,
                                    '.' => Tile::Space,
                                    _ => panic!("ERROR: bad input {c}"),
                                },
                                0, // Each tile carry additional information whether it is visited or not
                            )
                        })
                        .collect()
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-16-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 46);
        // assert_eq!(part_2(TEST_INPUT).unwrap(), 51);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-16.txt")).unwrap(),
            7_034
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-16.txt")).unwrap(),
            7_759
        );
    }
}
