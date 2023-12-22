use aoc_2023_lib::main;
use itertools::Itertools;

use std::{
    collections::{HashMap, HashSet},
    error::Error,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Bricks = Vec<(usize, usize, usize, usize, usize, usize, usize)>;
type Grid = HashMap<(usize, usize, usize), usize>;

main! {
    let input = include_str!("../../inputs/day-22.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

//https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/22.rs
fn part_1(input: &str) -> Result<usize> {
    let (bricks, grid) = generate_bricks(input);

    let mut above = HashMap::<_, HashSet<_>>::new();
    let mut below = HashMap::<_, HashSet<_>>::new();
    for &(x1, y1, z1, x2, y2, _, i) in &bricks {
        for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
            if let Some(&j) = grid.get(&(x, y, z1 - 1)) {
                above.entry(j).or_default().insert(i);
                below.entry(i).or_default().insert(j);
            }
        }
    }
    let mut above = HashMap::<_, HashSet<_>>::new();
    let mut below = HashMap::<_, HashSet<_>>::new();
    for &(x1, y1, z1, x2, y2, _, i) in &bricks {
        for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
            if let Some(&j) = grid.get(&(x, y, z1 - 1)) {
                above.entry(j).or_default().insert(i);
                below.entry(i).or_default().insert(j);
            }
        }
    }
    let mut p1 = 0;
    for b in 0..bricks.len() {
        let mut falling = HashSet::new();
        if_disintegrated(&mut falling, &above, &below, b);
        p1 += if falling.len() == 1 { 1 } else { 0 };
    }
    Ok(p1)
}

fn part_2(input: &str) -> Result<usize> {
    let (bricks, grid) = generate_bricks(input);
    let mut above = HashMap::<_, HashSet<_>>::new();
    let mut below = HashMap::<_, HashSet<_>>::new();
    for &(x1, y1, z1, x2, y2, _, i) in &bricks {
        for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
            if let Some(&j) = grid.get(&(x, y, z1 - 1)) {
                above.entry(j).or_default().insert(i);
                below.entry(i).or_default().insert(j);
            }
        }
    }
    let mut p2 = 0;
    for b in 0..bricks.len() {
        let mut falling = HashSet::new();
        if_disintegrated(&mut falling, &above, &below, b);
        p2 += falling.len() - 1;
    }
    Ok(p2)
}

fn parse(input: &str) -> Vec<(usize, usize, usize, usize, usize, usize, usize)> {
    input
        .split('\n')
        .enumerate()
        .map(|(i, l)| {
            let (a, b) = l.split_once('~').unwrap();
            let (x1, y1, z1) = a
                .split(',')
                .map(|w| w.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let (x2, y2, z2) = b
                .split(',')
                .map(|w| w.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            (x1, y1, z1, x2, y2, z2, i)
        })
        .collect::<Vec<_>>()
}

fn generate_bricks(input: &str) -> (Bricks, Grid) {
    let mut bricks = parse(input);
    let mut grid: HashMap<(usize, usize, usize), usize> = HashMap::new();
    for &(x1, y1, z1, x2, y2, z2, i) in &bricks {
        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    grid.insert((x, y, z), i);
                }
            }
        }
    }
    loop {
        let mut done = true;
        for b in &mut bricks {
            let mut can_fall = true;
            while can_fall {
                let (x1, y1, z1, x2, y2, z2, i) = *b;
                if z1 == 1 {
                    break;
                }
                for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
                    if grid.contains_key(&(x, y, z1 - 1)) {
                        can_fall = false;
                        break;
                    }
                }
                if can_fall {
                    *b = (x1, y1, z1 - 1, x2, y2, z2 - 1, i);
                    for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
                        grid.remove(&(x, y, z2));
                        grid.insert((x, y, z1 - 1), i);
                    }
                    done = false;
                }
            }
        }
        if done {
            break;
        }
    }
    (bricks, grid)
}

fn if_disintegrated(
    falling: &mut HashSet<usize>,
    above: &HashMap<usize, HashSet<usize>>,
    below: &HashMap<usize, HashSet<usize>>,
    brick: usize,
) {
    if !falling.insert(brick) {
        return;
    }
    let Some(parents) = above.get(&brick) else {
        return;
    };
    for &parent in parents {
        if below[&parent].iter().all(|x| falling.contains(x)) {
            if_disintegrated(falling, above, below, parent);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-22-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 5);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 7);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-22.txt")).unwrap(),
            389
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-22.txt")).unwrap(),
            70_609
        );
    }
}
