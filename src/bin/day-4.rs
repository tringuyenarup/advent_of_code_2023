use aoc_2023_lib::main;

use std::{collections::HashSet, error::Error, str::FromStr};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-4.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    let games = input
        .lines()
        .map(|game| game.parse::<Game>().unwrap())
        .collect::<Vec<Game>>();

    Ok(games
        .into_iter()
        .map(|game| {
            let count = game.winning_cards.intersection(&game.your_cards).count();
            if count == 0 {
                0
            } else {
                2_usize.pow((count as u32) - 1)
            }
        })
        .sum::<usize>())
}

fn part_2(input: &str) -> Result<i32> {
    let games = input
        .lines()
        .map(|game| game.parse::<Game>().unwrap())
        .collect::<Vec<Game>>();

    let ranges = games
        .iter()
        .map(|game| {
            let count = game.winning_cards.intersection(&game.your_cards).count() as i32;
            if count == 0 {
                (game.id, None)
            } else {
                (
                    game.id,
                    Some((1..=count).map(|x| x + game.id).collect::<HashSet<i32>>()),
                )
            }
        })
        .collect::<Vec<(i32, Option<HashSet<i32>>)>>();

    Ok(ranges
        .iter()
        .fold(vec![1; ranges.len()], |mut acc: Vec<i32>, (game_id, _)| {
            if *game_id != 1 {
                for other_id in 1..*game_id {
                    let (_, range) = &ranges[(other_id - 1) as usize];
                    if range.is_some() {
                        let range = range.clone().unwrap();
                        if range.contains(game_id) {
                            acc[(*game_id - 1) as usize] += acc[(other_id - 1) as usize]
                        }
                    }
                }
            }

            acc
        })
        .iter()
        .sum::<i32>())
}
#[derive(Debug)]
struct Game {
    id: i32,
    winning_cards: HashSet<i32>,
    your_cards: HashSet<i32>,
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (winning_cards, your_cards) = s.split_once(" | ").unwrap();
        let (id, winning_cards) = winning_cards.split_once(": ").unwrap();

        Ok(Self {
            id: id.split_ascii_whitespace().last().unwrap().parse().unwrap(),
            winning_cards: winning_cards
                .split_ascii_whitespace()
                .map(|card| card.parse::<i32>().unwrap())
                .collect(),
            your_cards: your_cards
                .split_ascii_whitespace()
                .map(|card| card.parse::<i32>().unwrap())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-4-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 13);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 30);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-4.txt")).unwrap(),
            21_959
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-4.txt")).unwrap(),
            5_132_675
        );
    }
}
