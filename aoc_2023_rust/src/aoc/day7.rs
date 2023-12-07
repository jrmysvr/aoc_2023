use crate::aoc::input::read_input_for_day;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

pub fn run() {
    println!("Day 7 Solutions");
    println!("---------------");
    let input = read_input_for_day(7);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

lazy_static! {
    static ref CARDS: [HashMap<char, Num>; 2] = [
        HashMap::from([
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('J', 11),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
        ]),
        HashMap::from([
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
            ('J', 1),
        ])
    ];
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

type Card = char;
type Num = i32;

#[derive(Debug, Ord, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: Num,
    hand_type: HandType,
    version: usize,
}

impl Hand {
    fn new(card_str: &str, bid: Num) -> Self {
        let cards = parse_cards_from(card_str);
        let hand_type = calc_hand_type_of(&cards);
        Self {
            cards: cards,
            bid: bid,
            hand_type: hand_type,
            version: 0,
        }
    }

    fn new2(card_str: &str, bid: Num) -> Self {
        let cards = parse_cards_from(card_str);
        let hand_type = calc_hand_type2_of(&cards);
        Self {
            cards: cards,
            bid: bid,
            hand_type: hand_type,
            version: 1,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        &self.hand_type == &other.hand_type && &self.cards == &other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cmp = if self.hand_type == other.hand_type {
            let self_values = self
                .cards
                .iter()
                .map(|card| CARDS[self.version][card])
                .collect::<Vec<Num>>();
            let other_values = other
                .cards
                .iter()
                .map(|card| CARDS[other.version][card])
                .collect::<Vec<Num>>();
            let order = self_values.cmp(&other_values);
            order
        } else {
            self.hand_type.cmp(&other.hand_type)
        };

        Some(cmp)
    }
}

fn calc_hand_type_of(cards: &Vec<Card>) -> HandType {
    let unique_cards: HashSet<Card> = HashSet::from_iter(cards.clone().into_iter());
    match unique_cards.len() {
        5 => HandType::HighCard,
        4 => HandType::OnePair,
        1 => HandType::FiveOfKind,
        2 | 3 => {
            let mut counts: HashMap<char, u8> = HashMap::new();
            for card in cards.iter() {
                counts.entry(*card).and_modify(|c| *c += 1).or_insert(1);
            }
            let mut values: Vec<u8> = counts.into_values().collect();
            values.sort_unstable();
            return match values[..] {
                [1, 4] => HandType::FourOfKind,
                [2, 3] => HandType::FullHouse,
                [1, 1, 3] => HandType::ThreeOfKind,
                [1, 2, 2] => HandType::TwoPair,
                _ => panic!("Unknown hand type from value: {values:?}"),
            };
        }
        _ => panic!("Invalid cards!"),
    }
}

fn calc_hand_type2_of(cards: &Vec<Card>) -> HandType {
    let j_cards = cards
        .iter()
        .filter(|&card| *card == 'J')
        .collect::<Vec<&Card>>();
    let other_cards = cards
        .iter()
        .filter(|&card| *card != 'J')
        .collect::<Vec<&Card>>();
    let unique_cards: HashSet<&Card> = HashSet::from_iter(other_cards.clone().into_iter());
    match j_cards.len() {
        0 => match unique_cards.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            1 => HandType::FiveOfKind,
            2 | 3 => {
                let mut counts: HashMap<char, u8> = HashMap::new();
                for card in cards.iter() {
                    counts.entry(*card).and_modify(|c| *c += 1).or_insert(1);
                }
                let mut values: Vec<u8> = counts.into_values().collect();
                values.sort_unstable();
                return match values[..] {
                    [1, 4] => HandType::FourOfKind,
                    [2, 3] => HandType::FullHouse,
                    [1, 1, 3] => HandType::ThreeOfKind,
                    [1, 2, 2] => HandType::TwoPair,
                    _ => panic!("Unknown hand type from value: {values:?}"),
                };
            }
            _ => panic!("Invalid cards!"),
        },
        1 => match unique_cards.len() {
            4 => HandType::OnePair,
            3 => HandType::ThreeOfKind,
            2 => {
                let mut counts: HashMap<&char, u8> = HashMap::new();
                for card in other_cards.into_iter() {
                    counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
                }
                let mut values: Vec<u8> = counts.into_values().collect();
                values.sort_unstable();
                return match values[..] {
                    [1, 3] => HandType::FourOfKind,
                    [2, 2] => HandType::FullHouse,
                    _ => panic!("Unknown hand type from value: {values:?}"),
                };
            }
            1 => HandType::FiveOfKind,
            _ => panic!("Invalid cards!"),
        },
        2 | 3 => match unique_cards.len() {
            3 => HandType::ThreeOfKind,
            2 => HandType::FourOfKind,
            1 => HandType::FiveOfKind,
            _ => panic!("Invalid cards!"),
        },
        _ => HandType::FiveOfKind,
    }
}

fn parse_cards_from(input: &str) -> Vec<Card> {
    let cards = input.chars().collect::<Vec<Card>>();
    cards
}

fn parse_hand_from(input: &str) -> Hand {
    let bid = Num::from_str_radix(input.split(' ').nth(1).unwrap().trim(), 10).unwrap();
    let card_str = input.split(' ').nth(0).unwrap().trim();
    Hand::new(card_str, bid)
}

fn parse_hand2_from(input: &str) -> Hand {
    let bid = Num::from_str_radix(input.split(' ').nth(1).unwrap().trim(), 10).unwrap();
    let card_str = input.split(' ').nth(0).unwrap().trim();
    Hand::new2(card_str, bid)
}

fn parse_hands_from(input: &String) -> Vec<Hand> {
    input
        .split('\n')
        .map(parse_hand_from)
        .collect::<Vec<Hand>>()
}

fn parse_hands2_from(input: &String) -> Vec<Hand> {
    input
        .split('\n')
        .map(parse_hand2_from)
        .collect::<Vec<Hand>>()
}

fn solve_part1(input: &String) -> String {
    let mut hands = parse_hands_from(input);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(ix, hand)| ((ix + 1) as Num) * hand.bid)
        .fold(0, |acc, h| acc + h)
        .to_string()
}

fn solve_part2(input: &String) -> String {
    let mut hands = parse_hands2_from(input);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(ix, hand)| ((ix + 1) as Num) * hand.bid)
        .fold(0, |acc, h| acc + h)
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_hands() {
        let input = get_input(0);
        let first_input = input.split('\n').nth(0).unwrap();
        let expected_hand = Hand::new("32T3K", 765);
        assert_eq!(parse_hand_from(&first_input), expected_hand);

        let first_two = input.split('\n').take(2).collect::<Vec<&str>>().join("\n");
        let expected_hands = vec![Hand::new("32T3K", 0), Hand::new("T55J5", 0)];
        let hands = parse_hands_from(&first_two);
        assert_eq!(hands.len(), expected_hands.len());
        for (actual, expected) in hands.iter().zip(expected_hands.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_parse_hands_part2() {
        let input = get_input(0);
        let first_input = input.split('\n').nth(0).unwrap();
        let expected_hand = Hand::new2("32T3K", 765);
        assert_eq!(parse_hand2_from(&first_input), expected_hand);

        let first_two = input.split('\n').take(2).collect::<Vec<&str>>().join("\n");
        let expected_hands = vec![Hand::new2("32T3K", 0), Hand::new2("T55J5", 0)];
        let hands = parse_hands2_from(&first_two);
        assert_eq!(hands.len(), expected_hands.len());
        for (actual, expected) in hands.iter().zip(expected_hands.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_parse_hand_types() {
        let hands = vec![
            Hand::new("2T3K3", 0),
            Hand::new("5T5J5", 0),
            Hand::new("22J22", 0),
        ];

        let expected_types = vec![
            HandType::OnePair,
            HandType::ThreeOfKind,
            HandType::FourOfKind,
        ];

        for (hand, expected) in hands.iter().zip(expected_types.iter()) {
            assert_eq!(hand.hand_type, *expected);
        }
    }

    #[test]
    fn test_parse_hand_types_part2() {
        let hands = vec![
            Hand::new2("2T3K3", 0),
            Hand::new2("T55J5", 0),
            Hand::new2("KK677", 0),
            Hand::new2("KTJJT", 0),
            Hand::new2("QQQJA", 0),
            Hand::new2("22J22", 0),
        ];

        let expected_types = vec![
            HandType::OnePair,
            HandType::FourOfKind,
            HandType::TwoPair,
            HandType::FourOfKind,
            HandType::FourOfKind,
            HandType::FiveOfKind,
        ];

        for (hand, expected) in hands.iter().zip(expected_types.iter()) {
            assert_eq!(hand.hand_type, *expected);
        }
    }

    #[test]
    fn test_compare_hands() {
        assert!(HandType::ThreeOfKind < HandType::FourOfKind);

        let hands = vec![
            // Win                  Lose
            (Hand::new("5T5J5", 0), Hand::new("2T3K3", 0)),
            (Hand::new("22J22", 0), Hand::new("5T5J5", 0)),
        ];
        for (win, lose) in hands {
            println!("{win:?} > {lose:?}");
            assert!(win > lose);
        }
    }

    #[test]
    fn test_compare_hands2() {
        assert!(HandType::ThreeOfKind < HandType::FourOfKind);

        let hands = vec![
            // Win                  Lose
            (Hand::new2("T55J5", 0), Hand::new2("23T3K", 0)),
            (Hand::new2("KK677", 0), Hand::new2("23T3K", 0)),
            (Hand::new2("KTJJT", 0), Hand::new2("QQQJA", 0)),
            (Hand::new2("22J22", 0), Hand::new2("T55J5", 0)),
        ];
        for (win, lose) in hands {
            println!("{win:?} > {lose:?}");
            assert!(win > lose);
        }
    }

    #[test]
    fn test_sort_hands() {
        let mut hands = vec![
            Hand::new("2T3K3", 0),
            Hand::new("5T5J5", 0),
            Hand::new("KK677", 0),
            Hand::new("KTJJT", 0),
            Hand::new("QQQJA", 0),
        ];

        let sorted_hands = vec![
            Hand::new("2T3K3", 0),
            Hand::new("KTJJT", 0),
            Hand::new("KK677", 0),
            Hand::new("5T5J5", 0),
            Hand::new("QQQJA", 0),
        ];
        hands.sort();

        for (actual, expected) in hands.iter().zip(sorted_hands.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_sort_hands2() {
        let mut hands = vec![
            Hand::new2("32T3K", 0),
            Hand::new2("T55J5", 0),
            Hand::new2("KK677", 0),
            Hand::new2("KTJJT", 0),
            Hand::new2("QQQJA", 0),
        ];

        let sorted_hands = vec![
            Hand::new2("32T3K", 0),
            Hand::new2("KK677", 0),
            Hand::new2("T55J5", 0),
            Hand::new2("QQQJA", 0),
            Hand::new2("KTJJT", 0),
        ];
        hands.sort();

        for (actual, expected) in hands.iter().zip(sorted_hands.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "6440");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "5905");
    }
}
