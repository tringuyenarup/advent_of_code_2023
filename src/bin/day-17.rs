use aoc_2023_lib::main;

use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-17.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    let grid = parse(input);
    let start = Point::new(0, 0);
    let end = Point::new(grid.len() - 1, grid[0].len() - 1);
    Ok(dijkstra(&grid, &start, &end, 1, 3, get_neighbours))
}

fn part_2(input: &str) -> Result<usize> {
    let grid = parse(input);
    let start = Point::new(0, 0);
    let end = Point::new(grid.len() - 1, grid[0].len() - 1);
    Ok(dijkstra(&grid, &start, &end, 4, 10, get_neighbours))
}
fn get_neighbours(
    node: &Node,
    grid: &[Vec<usize>],
    min_steps: usize,
    max_steps: usize,
) -> Vec<Node> {
    let mut neighbors = Vec::new();
    for (d, p) in node.position.valid_next(grid) {
        if d == node.direction.opposite() {
            // We can't go in the opposite direction.
            continue;
        } else if d != node.direction && node.direction_count >= min_steps {
            // We can only change direction if we've already gone in
            // this direction 4 times or more.
            neighbors.push(Node::new(d, p, 1));
        } else if d == node.direction && node.direction_count < max_steps {
            // We can only go in the same direction if we haven't gone
            // more than 10 times in this durection..
            neighbors.push(Node::new(d, p, node.direction_count + 1));
        }

        // All other cases are invalid for p2.
    }
    neighbors
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

fn dijkstra<F>(
    grid: &[Vec<usize>],
    start: &Point,
    end: &Point,
    min_steps: usize,
    max_steps: usize,
    neighbor_fn: F,
) -> usize
where
    F: Fn(&Node, &[Vec<usize>], usize, usize) -> Vec<Node>,
{
    // Track our min distances at each Node. In our specific case, we
    // have multiple because we could be coming from South or East
    // at the start.
    let mut distances = HashMap::new();
    distances.insert(Node::new(Direction::South, start.clone(), 0), 0);
    distances.insert(Node::new(Direction::East, start.clone(), 0), 0);

    // Track paths we want to visit. Again, we are adding two because
    // we could be coming from either.
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        cost: 0,
        node: Node::new(Direction::South, start.clone(), 0),
    });
    frontier.push(State {
        cost: 0,
        node: Node::new(Direction::East, start.clone(), 0),
    });

    // Grab the next node from the frontier.
    while let Some(State { cost, node }) = frontier.pop() {
        // If we are at the goal, we are done.
        if end.x == node.position.x && end.y == node.position.y {
            return cost;
        }

        // Otherwise, check our neighbors.
        for neighbor in neighbor_fn(&node, grid, min_steps, max_steps) {
            // If we've already visited this node and it was cheaper,
            // we don't need to keep checking this way.
            let new_cost = cost + grid[neighbor.position.y][neighbor.position.x];
            if let Some(&best) = distances.get(&neighbor) {
                if new_cost >= best {
                    continue;
                }
            }

            // Otherwise, add it to our distances and frontier.
            distances.insert(neighbor.clone(), new_cost);
            frontier.push(State {
                cost: new_cost,
                node: neighbor,
            });
        }
    }

    // If we get here, we didn't find a path. Not possible in our
    // case, but is in the general case.
    unreachable!("ERROR: THERE MUST BE AN ANSWER")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    cost: usize,
    node: Node,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We are using a min heap, so we are doing this backwards.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    position: Point,
    direction: Direction,
    direction_count: usize,
}

impl Node {
    fn new(direction: Direction, position: Point, direction_count: usize) -> Self {
        Self {
            position,
            direction,
            direction_count,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    // Return the opposite direction.
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    // Return the valid next points from this point. It won't include
    // any that are out of bounds.
    fn valid_next(&self, grid: &[Vec<usize>]) -> Vec<(Direction, Point)> {
        let mut next = Vec::new();
        if self.x > 0 {
            next.push((Direction::West, Self::new(self.x - 1, self.y)));
        }
        if self.y > 0 {
            next.push((Direction::North, Self::new(self.x, self.y - 1)));
        }
        if self.x < grid[0].len() - 1 {
            next.push((Direction::East, Self::new(self.x + 1, self.y)));
        }
        if self.y < grid.len() - 1 {
            next.push((Direction::South, Self::new(self.x, self.y + 1)));
        }
        next
    }
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
