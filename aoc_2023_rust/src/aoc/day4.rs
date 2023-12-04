#![allow(dead_code)]
#![allow(unused_variables)]

use crate::aoc::input::read_input_for_day;
use std::collections::HashSet;

pub fn run() {
    println!("Day 4 Solutions");
    println!("---------------");
    let input = read_input_for_day(4);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Num = i32;
type Wins = HashSet<Num>;
type Nums = Vec<Num>;
type Cards = Vec<(Wins, Nums)>;

fn convert_input_to_cards(input: &String) -> Cards {
    let mut cards = Cards::new();
    for line in input.trim().split('\n') {
        let all_numbers = line.split(':').nth(1).unwrap();
        let winning = all_numbers
            .split(' ')
            .filter(|&s| s != "")
            .take_while(|&s| s != "|")
            .map(|w| Num::from_str_radix(w, 10).unwrap())
            .collect::<Wins>();
        let numbers = all_numbers
            .split(' ')
            .filter(|&s| s != "")
            .skip_while(|&s| s != "|")
            .skip(1)
            .map(|n| Num::from_str_radix(n, 10).unwrap())
            .collect::<Nums>();
        cards.push((winning, numbers));
    }

    cards
}

fn solve_part1(input: &String) -> String {
    let cards = convert_input_to_cards(input);
    let mut points = 0;
    for (winning, numbers) in cards {
        let count = numbers
            .iter()
            .filter(|num| winning.contains(num))
            .collect::<Vec<&Num>>()
            .len();
        let point = if count > 0 {
            2u32.pow((count as i32 - 1) as u32)
        } else {
            0
        };
        points += point;
    }

    points.to_string()
}

fn solve_part2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: [&str; 1] = ["
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_convert_input_to_cards() {
        let input = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let expected_cards: Cards = vec![(Wins::from([41, 48, 83, 86, 17]), vec![])];
        let cards = convert_input_to_cards(&input);

        assert!(cards.len() == 1);

        for (actual, expected) in cards.iter().zip(expected_cards.iter()) {
            let (a_win, _) = actual;
            let (e_win, _) = expected;
            assert_eq!(a_win, e_win);
        }
    }

    #[test]
    fn test_full() {
        assert_eq!(solve_part1(&get_input(0)), "13");
    }
}
