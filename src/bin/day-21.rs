use aoc_2023_lib::main;
use itertools::Itertools;
use pathfinding::matrix::Matrix;

use std::{collections::HashSet, error::Error, str::FromStr};
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
    Ok(part(input, 26_501_365))
}

fn part(input: &str, goal: usize) -> usize {
    let grid = Matrix::from_rows(input.lines().map(str::bytes)).unwrap();
    let (sr, sc) = grid
        .items()
        .find_map(|(pos, b)| (*b == b'S').then_some(pos))
        .unwrap();
    let (g, mut ys, mut reachable) = (grid.rows, vec![], HashSet::new());
    reachable.insert((sr as isize, sc as isize));
    for count in 1..=goal {
        for (r, c) in reachable.drain().collect::<Vec<_>>() {
            reachable.extend(
                [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                    .iter()
                    .filter(|&&(nr, nc)| grid[grid.constrain((nr, nc))] != b'#'),
            );
        }
        if count % g == g / 2 {
            ys.push(reachable.len());
            if let &[y0, y1, y2] = &ys[..] {
                let x = goal / g;
                return (x * x * (y0 + y2 - 2 * y1) + x * (4 * y1 - 3 * y0 - y2) + 2 * y0) / 2;
            }
        }
    }
    reachable.len()
}

struct Map {
    start: (i32, i32),
    gardens: HashSet<(i32, i32)>,
}

impl Map {
    fn step(&self, steps: isize) -> usize {
        let mut queue = Vec::new();
        queue.push(self.start);
        let mut counter = 0;

        while counter < steps {
            let mut new_queue = HashSet::new();
            while let Some(pos) = queue.pop() {
                let new_pos = [
                    (pos.0 + 1, pos.1),
                    (pos.0 - 1, pos.1),
                    (pos.0, pos.1 + 1),
                    (pos.0, pos.1 - 1),
                ]
                .iter()
                .filter_map(|(new_r, new_c)| {
                    if self.gardens.contains(&(*new_r, *new_c)) {
                        Some((*new_r, *new_c))
                    } else {
                        None
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

        let gardens = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(col, c)| match c {
                        '.' => Some((row as i32, col as i32)),
                        'S' => {
                            start = (row as i32, col as i32);
                            Some((row as i32, col as i32))
                        }
                        _ => None,
                    })
                    .collect_vec()
            })
            .collect::<HashSet<(i32, i32)>>();
        Ok(Self { start, gardens })
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
        assert_eq!(
            part_2(include_str!("../../inputs/day-21.txt")).unwrap(),
            584_211_423_220_706
        );
    }
}
