use aoc_2023_lib::main;
use itertools::Itertools;
use z3::ast::{Ast, Int};

use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-24.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<i32> {
    Ok(input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" @ ").unwrap();
            let left = left
                .split(", ")
                .map(|n| n.parse::<f64>().unwrap())
                .collect_vec();
            let x = left[0];
            let y = left[1];

            let right = right
                .split(", ")
                .map(|n| {
                    n.split_ascii_whitespace()
                        .next()
                        .unwrap()
                        .parse::<isize>()
                        .unwrap()
                })
                .collect_vec();

            let vx = right[0];
            let vy = right[1];

            let slope = (vy as f64 / vx as f64);

            (slope, y - x * slope, x, vx as f64)
        })
        .tuple_combinations()
        .filter_map(|(d1, d2)| {
            if d1.0 == d2.0 {
                None
            } else {
                let x = (d2.1 - d1.1) / (d1.0 - d2.0);
                let y = -(d2.0 * d1.1 - d2.1 * d1.0) / (d1.0 - d2.0);

                if (200_000_000_000_000f64..=400_000_000_000_000f64).contains(&x)
                    && (200_000_000_000_000f64..=400_000_000_000_000f64).contains(&y)
                {
                    if (d1.3 < 0f64 && x > d1.2)
                        || (d1.3 > 0f64 && x < d1.2)
                        || (d2.3 < 0f64 && x > d2.2)
                        || (d2.3 > 0f64 && x < d2.2)
                    {
                        return None;
                    }
                    Some(1)
                } else {
                    None
                }
            }
        })
        .sum())
}

fn part_2(input: &str) -> Result<i64> {
    let hail_stones = input
        .lines()
        .map(|line| {
            let (pos, d_pos) = line.split_once(" @ ").unwrap();
            (
                pos.split(", ")
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap(),
                d_pos
                    .trim()
                    .split(", ")
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .collect::<Vec<((i64, i64, i64), (i64, i64, i64))>>();

    let ctx = z3::Context::new(&z3::Config::new());
    let solver = z3::Solver::new(&ctx);
    let [fx, fy, fz, fdx, fdy, fdz] =
        ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Int::new_const(&ctx, v));

    let zero = Int::from_i64(&ctx, 0);
    for (i, &((x, y, z), (dx, dy, dz))) in hail_stones.iter().enumerate() {
        let [x, y, z, dx, dy, dz] = [x, y, z, dx, dy, dz].map(|v| Int::from_i64(&ctx, v as _));
        let t = Int::new_const(&ctx, format!("t{i}"));
        solver.assert(&t.ge(&zero));
        solver.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        solver.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        solver.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }

    assert_eq!(solver.check(), z3::SatResult::Sat);
    let model = solver.get_model().unwrap();
    let res = model.eval(&(&fx + &fy + &fz), true).unwrap();
    Ok(res.as_i64().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-24-test.txt");
    // #[test]
    // fn test_input() {
    //     // assert_eq!(part_1(TEST_INPUT).unwrap(), 2); // this one will fail because it ranges are different
    //     // assert_eq!(part_2(TEST_INPUT).unwrap(), 47);
    // }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-24.txt")).unwrap(),
            29_142
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-24.txt")).unwrap(),
            848_947_587_263_033
        );
    }
}
