use aoc_2023_lib::main;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use rustworkx_core::{connectivity::stoer_wagner_min_cut, petgraph::prelude::UnGraph};

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    vec,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-25.txt");
    (part_1(input).unwrap(), '🎄')
}

fn part_1(input: &str) -> Result<usize> {
    let mut graph = rustworkx_core::petgraph::Graph::new_undirected();
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(": ");
        let node = parts.next().unwrap();
        let node = *nodes.entry(node).or_insert_with(|| graph.add_node(node));
        let edges = parts.next().unwrap().split(' ');
        for edge in edges {
            let edge = *nodes.entry(edge).or_insert_with(|| graph.add_node(edge));
            graph.add_edge(node, edge, ());
        }
    }
    println!("rustworkx:");
    let now = std::time::Instant::now();
    let ans = match stoer_wagner_min_cut(&graph, |_| Ok::<i32, ()>(1)) {
        Err(_) => unreachable!(),
        Ok(None) => panic!("ERROR: no solution found"),
        Ok(Some((cut, partition))) => {
            println!("rustworkx-minmum-cut: {}", cut);
            partition.len() * (nodes.len() - partition.len())
        }
    };
    println!("p1-rustworkx: {} ({:?})", ans, now.elapsed());
    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-25-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 54);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-25.txt")).unwrap(),
            552_695
        );
    }
}
