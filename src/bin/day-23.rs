use aoc_2023_lib::main;
use itertools::Itertools;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Point = (usize, usize);
main! {
    let input = include_str!("../../inputs/day-23.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    let grid = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
    let mut ans = 0;
    dfs(&grid, &mut seen, (0, 1), 0, &mut ans);

    Ok(ans)

    // Ok(bdf(&grid))
}

fn part_2(input: &str) -> Result<usize> {
    let grid = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let edges = find_branching_edges(&grid);
    let mut seen = HashSet::new();
    let mut ans = 0;
    dfs_branches(&grid, &edges, (0, 1), &mut seen, 0, &mut ans);
    Ok(ans)
    // Ok(bfs_branches(&grid))
}

fn dfs_branches(
    grid: &[&[u8]],
    edges: &HashMap<Point, Vec<(Point, usize)>>,
    pos: (usize, usize),
    seen: &mut HashSet<(usize, usize)>,
    dist: usize,
    max_dist: &mut usize,
) {
    if pos.0 == grid.len() - 1 {
        *max_dist = (*max_dist).max(dist);
    }
    if !seen.insert(pos) {
        return;
    }
    let neighbours = edges.get(&pos).unwrap();
    for &(n, cost) in neighbours {
        if seen.contains(&n) {
            continue;
        }
        dfs_branches(grid, edges, n, seen, dist + cost, max_dist);
    }
    seen.remove(&pos);
}

fn bfs_branches(grid: &[&[u8]]) -> usize {
    let edges = find_branching_edges(grid);
    let mut ends = Vec::new();
    let visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(((0, 1), visited.clone(), 0));
    while let Some((pos, mut visited, cost)) = queue.pop_front() {
        if pos.0 == grid.len() - 1 {
            ends.push(cost);
            continue;
        }
        if !visited.insert(pos) {
            continue;
        }
        for (n, c) in edges
            .get(&pos)
            .unwrap()
            .iter()
            .filter(|&n| !visited.contains(&n.0))
        {
            queue.push_back((*n, visited.clone(), cost + c));
        }
    }

    *ends.iter().max().unwrap()
}

fn find_neighbours(grid: &[&[u8]], pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    for &(dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice() {
        let rr = (pos.0 as isize + dr) as usize;
        let cc = (pos.1 as isize + dc) as usize;
        let Some(&tile) = grid.get(rr).and_then(|row| row.get(cc)) else {
            continue;
        };
        if tile == b'#' {
            continue;
        }
        neighbours.push((rr, cc));
    }
    neighbours
}

fn find_branching_edges(grid: &[&[u8]]) -> HashMap<Point, Vec<(Point, usize)>> {
    let map = grid
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter().enumerate().filter_map(move |(col, tile)| {
                if *tile != b'#' {
                    let num_of_neighbours = find_neighbours(grid, &(row, col)).len();
                    Some(((row, col), num_of_neighbours))
                } else {
                    None
                }
            })
        })
        .collect::<HashMap<(usize, usize), usize>>();

    let nodes = map
        .iter()
        .filter_map(|(p, v)| if *v != 2 { Some(*p) } else { None })
        .collect::<HashSet<(usize, usize)>>();

    let mut edges = HashMap::<Point, Vec<(Point, usize)>>::new();

    for node in nodes {
        for mut neighbour in find_neighbours(grid, &node) {
            let mut prev = node;
            let mut dist = 0;
            loop {
                dist += 1;
                let neighbours = find_neighbours(grid, &neighbour);
                let neighbours = neighbours.iter().filter(|n| **n != prev).collect_vec();

                if neighbours.len() != 1 {
                    edges.entry(node).or_default().push((neighbour, dist));
                    break;
                }

                prev = neighbour;
                neighbour = *neighbours[0];
            }
        }
    }

    edges
}

fn dfs(
    grid: &[&[u8]],
    seen: &mut Vec<Vec<bool>>,
    (r, c): (usize, usize),
    dist: usize,
    max_dist: &mut usize,
) {
    if r == grid.len() - 1 {
        *max_dist = (*max_dist).max(dist);
    }

    let neighbours = match grid[r][c] {
        b'.' => [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice(),
        b'^' => [(-1, 0)].as_slice(),
        b'>' => [(0, 1)].as_slice(),
        b'v' => [(1, 0)].as_slice(),
        b'<' => [(0, -1)].as_slice(),
        _ => panic!("ERROR: BAD INPUT"),
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
        dfs(grid, seen, (rr, cc), dist + 1, max_dist);
        seen[rr][cc] = false;
    }
}

fn bdf(grid: &[&[u8]]) -> usize {
    let mut ends = Vec::<usize>::new();
    let mut queue = VecDeque::new();
    let seen = HashSet::new();

    queue.push_back(((0, 1), seen.clone(), 0));

    while let Some((pos, mut seen, cost)) = queue.pop_front() {
        if pos.0 == grid.len() - 1 {
            ends.push(cost);
            continue;
        }

        if !seen.insert(pos) {
            continue;
        }

        let neighbours = match grid[pos.0][pos.1] {
            b'.' => [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice(),
            b'^' => [(-1, 0)].as_slice(),
            b'>' => [(0, 1)].as_slice(),
            b'v' => [(1, 0)].as_slice(),
            b'<' => [(0, -1)].as_slice(),
            _ => panic!("ERROR: BAD INPUT"),
        };

        for &(dr, dc) in neighbours {
            let rr = (pos.0 as isize + dr) as usize;
            let cc = (pos.1 as isize + dc) as usize;
            let Some(&tile) = grid.get(rr).and_then(|row| row.get(cc)) else {
                continue;
            };
            if tile == b'#' || seen.contains(&(rr, cc)) {
                continue;
            }
            queue.push_back(((rr, cc), seen.clone(), cost + 1));
        }
    }

    ends.into_iter().max().unwrap()
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
        assert_eq!(
            part_2(include_str!("../../inputs/day-23.txt")).unwrap(),
            6_302
        );
    }
}
