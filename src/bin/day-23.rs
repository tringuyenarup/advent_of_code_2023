use aoc_2023_lib::main;

use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-23.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    let grid = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
    let mut ans = 0;
    dfs(&grid, &mut seen, (0, 1), 0, &mut ans, false);
    Ok(ans)
}

fn part_2(input: &str) -> Result<usize> {
    let grid = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
    let mut ans = 0;
    dfs(&grid, &mut seen, (0, 1), 0, &mut ans, true);
    Ok(ans)
}

fn dfs(
    grid: &[&[u8]],
    seen: &mut Vec<Vec<bool>>,
    (r, c): (usize, usize),
    dist: usize,
    max_dist: &mut usize,
    part2: bool,
) {
    if r == grid.len() - 1 {
        *max_dist = (*max_dist).max(dist);
    }

    let neighbours = match part2 {
        true => [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice(),
        false => match grid[r][c] {
            b'.' => [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice(),
            b'^' => [(-1, 0)].as_slice(),
            b'>' => [(0, 1)].as_slice(),
            b'v' => [(1, 0)].as_slice(),
            b'<' => [(0, -1)].as_slice(),
            _ => unreachable!(),
        },
    };
    for &(dr, dc) in neighbours {
        let rr = (r as isize + dr) as usize;
        let cc = (c as isize + dc) as usize;
        let Some(&tile) = grid.get(rr).and_then(|row| row.get(cc)) else {
            continue;
        };
        if tile == b'#' || seen[rr][cc] {
            continue;
        }
        seen[rr][cc] = true;
        dfs(grid, seen, (rr, cc), dist + 1, max_dist, part2);
        seen[rr][cc] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-23-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 94);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 154);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-23.txt")).unwrap(),
            2_034
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../../inputs/day-23.txt")).unwrap(), 11);
    }
}
