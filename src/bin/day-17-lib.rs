use aoc_2023_lib::{err, main};
use pathfinding::prelude::dijkstra;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-17.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    let graph = parse(input);
    let start = Node {
        position: (0, 0),
        direction: Direction::East,
        direction_count: 1,
    };
    let end = (graph.len() - 1, graph[0].len() - 1);

    let Some((_, res)) = dijkstra(
        &start,
        |node| successors(node, &graph, 1, 3),
        |node| node.position.0 == end.0 && node.position.1 == end.1,
    ) else {
        panic!("no solution found!");
    };

    Ok(res)
}

fn part_2(input: &str) -> Result<usize> {
    let graph = parse(input);
    let start = Node {
        position: (0, 0),
        direction: Direction::East,
        direction_count: 1,
    };
    let end = (graph.len() - 1, graph[0].len() - 1);

    let Some((_, res)) = dijkstra(
        &start,
        |node| successors(node, &graph, 4, 10),
        |node| node.position.0 == end.0 && node.position.1 == end.1,
    ) else {
        panic!("no solution found!");
    };

    Ok(res)
}

fn successors(node: &Node, graph: &[Vec<usize>], min: usize, max: usize) -> Vec<(Node, usize)> {
    let mut neighbours = Vec::<(Node, usize)>::new();
    for (direction, (new_x, new_y)) in valid_next(node.position, graph) {
        if direction == node.direction.opposite() {
            continue;
        }

        if direction != node.direction && node.direction_count >= min {
            neighbours.push((
                Node {
                    position: (new_x, new_y),
                    direction,
                    direction_count: 1,
                },
                graph[new_y][new_x],
            ));
        } else if direction == node.direction && node.direction_count < max {
            neighbours.push((
                Node {
                    position: (new_x, new_y),
                    direction,
                    direction_count: node.direction_count + 1,
                },
                graph[new_y][new_x],
            ));
        }
    }
    neighbours
}

fn valid_next(p: (usize, usize), grid: &[Vec<usize>]) -> Vec<(Direction, (usize, usize))> {
    let mut next = Vec::new();
    if p.0 > 0 {
        next.push((Direction::West, (p.0 - 1, p.1)));
    }
    if p.1 > 0 {
        next.push((Direction::North, (p.0, p.1 - 1)));
    }
    if p.0 < grid[0].len() - 1 {
        next.push((Direction::East, (p.0 + 1, p.1)));
    }
    if p.1 < grid.len() - 1 {
        next.push((Direction::South, (p.0, p.1 + 1)));
    }
    next
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    position: (usize, usize),
    direction: Direction,
    direction_count: usize,
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    East,
    North,
    West,
    South,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-17-test.txt");

    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 102);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 94);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-17.txt")).unwrap(),
            902
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-17.txt")).unwrap(),
            1_073
        );
    }
}
