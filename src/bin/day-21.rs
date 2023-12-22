use aoc_2023_lib::main;

use std::{collections::HashSet, error::Error};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-21.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}
//https://nickymeuleman.netlify.app/garden/aoc2023-day21
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    col: i64,
    row: i64,
}

impl Coord {
    fn neighbours(&self, rows: i64, cols: i64) -> Vec<Self> {
        let mut res = Vec::new();
        // up
        if self.row > 0 {
            res.push(Coord {
                col: self.col,
                row: self.row - 1,
            });
        }
        // down
        if self.row < rows - 1 {
            res.push(Coord {
                col: self.col,
                row: self.row + 1,
            });
        }
        // left
        if self.col > 0 {
            res.push(Coord {
                col: self.col - 1,
                row: self.row,
            });
        }
        // right
        if self.col < cols - 1 {
            res.push(Coord {
                col: self.col + 1,
                row: self.row,
            })
        };

        res
    }

    fn infinite_neighbours(&self) -> Vec<Self> {
        vec![
            Coord {
                col: self.col,
                row: self.row - 1,
            },
            Coord {
                col: self.col,
                row: self.row + 1,
            },
            Coord {
                col: self.col - 1,
                row: self.row,
            },
            Coord {
                col: self.col + 1,
                row: self.row,
            },
        ]
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Garden,
    Rock,
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Coord) {
    let mut start = Coord { col: 0, row: 0 };
    let mut grid = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Garden,
                '#' => Tile::Rock,
                'S' => {
                    start.col = x as i64;
                    start.row = y as i64;
                    Tile::Garden
                }
                _ => panic!(),
            };
            row.push(tile);
        }
        grid.push(row);
    }
    (grid, start)
}

pub fn part_1(input: &str) -> Result<usize> {
    let (grid, start) = parse(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut set = HashSet::new();
    set.insert(start);

    for _ in 0..64 {
        let mut new_set = HashSet::new();
        for pos in set {
            for n in pos
                .neighbours(rows as i64, cols as i64)
                .into_iter()
                .filter(|pos| grid[pos.row as usize][pos.col as usize] == Tile::Garden)
            {
                new_set.insert(n);
            }
        }
        set = new_set
    }
    Ok(set.len())
}

// Let f(n) be the number of spaces you can reach after n steps. Let X be the length of your input grid. f(n), f(n+X), f(n+2X), ...., is a quadratic
// You can find it by finding the first 3 values, then use that to interpolate the final answer.
pub fn part_2(input: &str) -> Result<usize> {
    let goal = 26_501_365;
    let (grid, start) = parse(input);
    let size = grid.len();
    // the amount of steps it takes to reach an edge of the map (all tiles in the same row and column as start are gardens)
    let to_edge = size / 2;
    let mut fn_results = Vec::new();
    let mut set = HashSet::new();
    set.insert(start);

    for count in 1.. {
        let mut new_set = HashSet::new();

        for pos in set {
            for n in pos.infinite_neighbours().into_iter().filter(|pos| {
                let y = pos.row.rem_euclid(size as i64) as usize;
                let x = pos.col.rem_euclid(size as i64) as usize;
                grid[y][x] == Tile::Garden
            }) {
                new_set.insert(n);
            }
        }
        set = new_set;

        if count == to_edge + size * fn_results.len() {
            fn_results.push(set.len());

            if fn_results.len() == 3 {
                // EITHER
                // let delta0 = fn_results[0];
                // let delta1 = fn_results[1] - fn_results[0];
                // let delta2 = fn_results[2] - 2 * fn_results[1] + fn_results[0];

                // return delta0
                //     + delta1 * (goal / size)
                //     + delta2 * ((goal / size) * ((goal / size) - 1) / 2);

                // OR, written differently:
                let n = goal / size;

                let a0 = fn_results[0];
                let a1 = fn_results[1];
                let a2 = fn_results[2];

                let b0 = a0;
                let b1 = a1 - a0;
                let b2 = a2 - a1;
                return Ok(b0 + b1 * n + (n * (n - 1) / 2) * (b2 - b1));
            }
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-21-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 42);
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
        assert_eq!(
            part_2(include_str!("../../inputs/day-21.txt")).unwrap(),
            584_211_423_220_706
        );
    }
}
