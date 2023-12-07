use aoc_2023_lib::{err, main};
use itertools::Itertools;

use std::{cmp::Ordering, collections::HashMap, error::Error, str::FromStr};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-7.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    let mut deck: Deck = Deck::new(input, false);

    deck.hands.sort_unstable();

    Ok(deck
        .hands
        .iter()
        .enumerate()
        .fold(0, |mut acc: usize, (index, hand)| {
            acc += hand.bet as usize * (index + 1);
            acc
        }))
}

fn part_2(input: &str) -> Result<usize> {
    let mut deck: Deck = Deck::new(input, true);

    deck.hands.sort_unstable();
    Ok(deck
        .hands
        .iter()
        .enumerate()
        .fold(0, |mut acc: usize, (index, hand)| {
            acc += hand.bet as usize * (index + 1);
            acc
        }))
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}
#[derive(Debug)]
struct Deck {
    hands: Vec<Hand>,
}

impl Deck {
    fn new(input: &str, joker_mode_on: bool) -> Self {
        let hands = input
            .lines()
            .map(|line| {
                let mut hand = line.parse::<Hand>().unwrap();
                if joker_mode_on {
                    hand.joker_mode = joker_mode_on;
                    hand.change_hand_type_by_joker()
                }
                hand
            })
            .collect::<Vec<Hand>>();
        Self { hands }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<char>,
    bet: u32,
    hand_type: HandType,
    joker_mode: bool,
}

impl Hand {
    fn get_point_by_hand_type(&self) -> u32 {
        match self.hand_type {
            HandType::FiveOfKind => 7,
            HandType::FourOfKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }

    fn get_point_by_card(&self, card: char) -> u32 {
        match card {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => match self.joker_mode {
                false => 11,
                true => 1,
            },
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("bad card"),
        }
    }

    fn change_hand_type_by_joker(&mut self) {
        let card_counts = self.cards.iter().counts();
        let num_of_jokers = card_counts.get(&'J').unwrap_or(&0);
        //  contains the counts for other types of cards
        let counts = card_counts
            .iter()
            .filter(|&(&k, _)| *k != 'J')
            .map(|(_, v)| *v)
            .collect::<Vec<usize>>();
        // let current_handtype = self.hand_type.clone();

        self.hand_type = match (*counts.iter().max().unwrap_or(&0), *num_of_jokers) {
            (a, b) if a + b == 5 => HandType::FiveOfKind,
            (a, b) if a + b == 4 => HandType::FourOfKind,
            (_, 0) => self.hand_type,
            (2, _) => {
                let pairs = counts.iter().filter(|&&v| v == 2).count();
                match (pairs, *num_of_jokers) {
                    (2, 1) => HandType::FullHouse,
                    (1, 1) => HandType::ThreeOfKind,
                    _ => panic!("this case should not appear here"),
                }
            }
            (1, 2) => HandType::ThreeOfKind,
            (1, 1) => HandType::OnePair,
            _ => panic!("this case should not appear here"),
        };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let us_point_by_hand = self.get_point_by_hand_type();
        let other_point_by_hand = other.get_point_by_hand_type();
        match us_point_by_hand.cmp(&other_point_by_hand) {
            Ordering::Greater | Ordering::Less => us_point_by_hand.cmp(&other_point_by_hand),
            Ordering::Equal => {
                for index in 0..self.cards.len() {
                    let us_point_by_card = self.get_point_by_card(self.cards[index]);
                    let other_point_by_card = self.get_point_by_card(other.cards[index]);
                    if us_point_by_card != other_point_by_card {
                        return us_point_by_card.cmp(&other_point_by_card);
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bet == other.bet && self.hand_type == other.hand_type
    }
}

impl Eq for Hand {}

impl FromStr for Hand {
    type Err = Box<dyn Error>;

    fn from_str(line: &str) -> Result<Self> {
        let (cards, bet) = line.split_once(' ').unwrap();
        let cards = cards.chars().collect_vec();
        // cards.reverse();

        let mut appearances = cards
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<char, usize>, char| {
                *acc.entry(*char).or_insert(0) += 1;
                acc
            })
            .values()
            .copied()
            .collect_vec();

        appearances.sort_unstable();
        appearances.reverse();

        let hand_type = match appearances.len() {
            1 => HandType::FiveOfKind,
            2 => {
                if appearances.contains(&4) {
                    HandType::FourOfKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if appearances.contains(&3) {
                    HandType::ThreeOfKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => return err!("bad hand type"),
        };
        Ok(Self {
            cards,
            bet: bet.parse().unwrap(),
            hand_type,
            joker_mode: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-7-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 6_440);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 5_905);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-7.txt")).unwrap(),
            250_602_641
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-7.txt")).unwrap(),
            251_037_509
        );
    }
}
