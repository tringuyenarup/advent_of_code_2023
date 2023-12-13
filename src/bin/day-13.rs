use aoc_2023_lib::main;

use std::{error::Error, usize};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-13.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}
// brute force everything.
// start from row 0 -> second to last row.
// then start from col 0 -> second to last col.
// target is number of different tile (0 part 1 and 1 part 2)

fn part_1(input: &str) -> Result<usize> {
    let grids = input
        .split("\n\n")
        .map(|s| s.split('\n').map(|l| l.as_bytes()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok(solve(&grids, 0))
}

fn part_2(input: &str) -> Result<usize> {
    let grids = input
        .split("\n\n")
        .map(|s| s.split('\n').map(|l| l.as_bytes()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok(solve(&grids, 1))
}
fn find_col(grid: &[&[u8]], limit: usize) -> Option<usize> {
    (0..grid[0].len() - 1).find(|&c| {
        let num_of_diff_tiles = (0..=c.min(grid[0].len() - c - 2))
            .map(|dc| {
                let a = c - dc;
                let b = c + 1 + dc;
                (0..grid.len())
                    .filter(|&r| grid[r][a] != grid[r][b])
                    .count()
            })
            .sum::<usize>();
        num_of_diff_tiles == limit
    })
}

fn find_row(grid: &[&[u8]], limit: usize) -> Option<usize> {
    (0..grid.len() - 1).find(|&r| {
        let num_of_diff_tiles = (0..=r.min(grid.len() - r - 2))
            .map(|dr| {
                let a = r - dr;
                let b = r + 1 + dr;
                (0..grid[0].len())
                    .filter(|&c| grid[a][c] != grid[b][c])
                    .count()
            })
            .sum::<usize>();
        num_of_diff_tiles == limit
    })
}

fn solve(grids: &[Vec<&[u8]>], limit: usize) -> usize {
    grids
        .iter()
        .map(|grid| {
            find_row(grid, limit)
                .map(|r| (r + 1) * 100)
                .or_else(|| find_col(grid, limit).map(|c| c + 1))
                .unwrap()
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-13-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 405);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 400);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-13.txt")).unwrap(),
            39_939
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-13.txt")).unwrap(),
            32_069
        );
    }
}
