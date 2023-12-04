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

fn convert_input_to_card_counts(input: &String) -> Vec<usize> {
    let mut results = Vec::<usize>::new();
    for line in input.trim().split('\n') {
        let all_numbers = line.split(':').nth(1).unwrap();
        let winning = all_numbers
            .split(' ')
            .filter(|&s| s != "")
            .take_while(|&s| s != "|")
            .map(|w| usize::from_str_radix(w, 10).unwrap())
            .collect::<HashSet<usize>>();
        let numbers = all_numbers
            .split(' ')
            .filter(|&s| s != "")
            .skip_while(|&s| s != "|")
            .skip(1)
            .map(|n| usize::from_str_radix(n, 10).unwrap())
            .collect::<Vec<usize>>();
        let count = numbers
            .iter()
            .filter(|num| winning.contains(num))
            .collect::<Vec<&usize>>()
            .len();
        results.push(count);
    }

    results
}

// Calculate total "points" for all winning cards
fn solve_part1(input: &String) -> String {
    let card_counts = convert_input_to_card_counts(input);
    let mut points = 0;
    for count in card_counts {
        points += if count > 0 {
            2u32.pow((count as i32 - 1) as u32)
        } else {
            0
        };
    }

    points.to_string()
}

// Calculate total number of winning cards which are won by winning cards!
fn solve_part2(input: &String) -> String {
    let card_counts = convert_input_to_card_counts(input);
    let mut won_cards = vec![0; card_counts.len()];
    // Cards are 1 indexed, but 0 indexing is nicer
    for card_id in 0..card_counts.len() {
        let count = card_counts[card_id];
        won_cards[card_id] += 1;
        // Copy won cards
        for i in 1..=count {
            won_cards[card_id + i] += won_cards[card_id];
        }
    }

    won_cards.iter().fold(0, |acc, c| acc + c).to_string()
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
    fn test_convert_input_to_card_counts() {
        let input = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        let expected_counts: Vec<usize> = vec![4];
        let counts = convert_input_to_card_counts(&input);

        assert_eq!(expected_counts.len(), counts.len());

        for (actual, expected) in counts.iter().zip(expected_counts.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "13");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "30");
    }
}
