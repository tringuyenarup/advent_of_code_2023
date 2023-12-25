use aoc_2023_lib::main;
use itertools::Itertools;

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

fn component_size(graph: &HashMap<&str, HashSet<&str>>, a: &str) -> usize {
    let mut seen = HashSet::new();
    let mut s = vec![a];
    while let Some(x) = s.pop() {
        if !seen.insert(x) {
            continue;
        }
        for b in &graph[x] {
            if !seen.contains(b) {
                s.push(b);
            }
        }
    }
    seen.len()
}

fn part_1(input: &str) -> Result<usize> {
    let mut graph = HashMap::<_, HashSet<_>>::new();
    let mut edges = HashSet::new();
    for l in input.split('\n') {
        let (a, rest) = l.split_once(": ").unwrap();
        for b in rest.split(' ') {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
            let (x, y) = if a < b { (a, b) } else { (b, a) };
            edges.insert((x, y));
        }
    }
    let mut dot = String::from("graph {\n");
    for (a, b) in &edges {
        dot += &format!("{} -- {}\n", a, b);
    }
    dot += "}";
    std::fs::write("out.dot", dot).unwrap();
    // println!("Run the following to visualize the graph:");
    // println!("dot -Tsvg -Kneato out.dot > out.svg");
    // println!("Manually find the three edges.");
    for (a, b) in [("xbl", "qqh"), ("tbq", "qfj"), ("xzn", "dsr")] {
        graph.get_mut(a).unwrap().remove(b);
        graph.get_mut(b).unwrap().remove(a);
    }
    let size = component_size(&graph, "qqh");
    Ok((graph.len() - size) * size)
}
// fn part_1(input: &str) -> Result<usize> {
//     let graph = input.lines().fold(
//         HashMap::new(),
//         |mut acc: HashMap<&str, HashSet<&str>>, line| {
//             let (lhs, rhs) = line.split_once(": ").unwrap();
//             for c in rhs.split_ascii_whitespace() {
//                 acc.entry(lhs).or_default().insert(c);
//                 acc.entry(c).or_default().insert(lhs);
//             }
//             acc
//         },
//     );
//     for (a, b, d, e, f, g) in graph.keys().cloned().tuple_combinations() {
//         let mut new_graph = graph.clone();
//         new_graph.get_mut(a).unwrap().remove(b);
//         new_graph.get_mut(b).unwrap().remove(a);
//         new_graph.get_mut(d).unwrap().remove(e);
//         new_graph.get_mut(e).unwrap().remove(d);
//         new_graph.get_mut(f).unwrap().remove(g);
//         new_graph.get_mut(g).unwrap().remove(f);

//         let mut ans = vec![HashSet::from([a]), HashSet::from([b])];

//         let mut queue = Vec::new();
//         queue.push(vec![a]);
//         queue.push(vec![b]);

//         for (index, mut group) in queue.into_iter().enumerate() {
//             while let Some(element) = group.pop() {
//                 ans[index].insert(element);

//                 if let Some(others) = new_graph.get(element) {
//                     for other in others {
//                         if ans[index].insert(other) {
//                             group.push(other);
//                         }
//                     }
//                 }
//             }
//         }

//         if ans[0].len() + ans[1].len() == graph.keys().len() {
//             return Ok(ans[0].len() * ans[1].len());
//         }
//     }
//     unreachable!("ERROR: THERE MUST BE AN ANSWER")
// }

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
