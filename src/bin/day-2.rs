use aoc_2023_lib::main;

use std::{error::Error, str::FromStr};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-2.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<i32> {
    let games = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .collect::<Vec<Game>>();

    Ok(games
        .iter()
        .filter(|g| {
            g.cubes.iter().all(|(cube, v)| match *cube {
                Cube::Red => *v <= 12,
                Cube::Green => *v <= 13,
                Cube::Blue => *v <= 14,
            })
        })
        .map(|g| g.id)
        .sum::<i32>())
}

fn part_2(input: &str) -> Result<i32> {
    let games = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .collect::<Vec<Game>>();

    Ok(games
        .iter()
        .map(|game| {
            game.cubes
                .iter()
                .fold(
                    vec![i32::MIN, i32::MIN, i32::MIN],
                    |mut acc, (cube, value)| {
                        match *cube {
                            Cube::Red => acc[0] = std::cmp::max(acc[0], *value),
                            Cube::Green => acc[1] = std::cmp::max(acc[1], *value),
                            Cube::Blue => acc[2] = std::cmp::max(acc[2], *value),
                        }
                        acc
                    },
                )
                .iter()
                .product::<i32>()
        })
        .sum::<i32>())
}

#[derive(Debug)]
struct Game {
    id: i32,
    cubes: Vec<(Cube, i32)>,
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(line: &str) -> Result<Self> {
        let (id, left) = line.split_once(": ").unwrap();
        let id = id.split_once(' ').unwrap().1.parse::<i32>().unwrap();
        Ok(Game {
            id,
            cubes: left
                .split("; ")
                .flat_map(|group| {
                    group.split(", ").map(|pair| {
                        let (value, t) = pair.split_once(' ').unwrap();
                        let cube = match t {
                            "red" => Cube::Red,
                            "blue" => Cube::Blue,
                            "green" => Cube::Green,
                            _ => panic!(),
                        };
                        (cube, value.parse::<i32>().unwrap())
                    })
                })
                .collect::<Vec<(Cube, i32)>>(),
        })
    }
}

#[derive(Debug)]
enum Cube {
    Red,
    Green,
    Blue,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 8);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 2286);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-2.txt")).unwrap(),
            3_059
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-2.txt")).unwrap(),
            65_371
        );
    }
}
