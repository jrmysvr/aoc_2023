use crate::aoc::input::read_input_for_day;

use std::collections::HashSet;

pub fn run() {
    println!("Day 9 Solutions");
    println!("---------------");
    let input = read_input_for_day(9);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Histories = Vec<History>;
type History = Vec<Num>;
type Num = i32;

fn parse_histories_from(input: &String) -> Histories {
    input
        .split('\n')
        .map(|line| {
            line.split(' ')
                .filter(|l| l.len() > 0)
                .map(|l| Num::from_str_radix(l, 10).unwrap())
                .collect::<History>()
        })
        .collect::<Histories>()
}
fn solve_part1(input: &String) -> String {
    let histories = parse_histories_from(input);
    let mut values = 0;
    for history in histories {
        let mut sequences = Histories::new();
        sequences.push(history.clone());
        let mut end = sequences.len() - 1;
        while HashSet::<&Num>::from_iter(sequences[end].iter()).len() != 1 {
            let diffs = (0..sequences[end].len() - 1)
                .map(|ix| sequences[end][ix + 1] - sequences[end][ix])
                .collect::<History>();
            sequences.push(diffs.clone());
            end = sequences.len() - 1;
        }
        values += sequences
            .iter()
            .map(|sequence| sequence[sequence.len() - 1])
            .fold(0, |acc, s| acc + s);
    }

    values.to_string()
}

fn solve_part2(input: &String) -> String {
    let histories = parse_histories_from(input);
    let mut values = 0;
    for history in histories {
        let mut sequences = Histories::new();
        sequences.push(history.clone());
        let mut end = sequences.len() - 1;
        while HashSet::<&Num>::from_iter(sequences[end].iter()).len() != 1 {
            let diffs = (0..sequences[end].len() - 1)
                .map(|ix| sequences[end][ix + 1] - sequences[end][ix])
                .collect::<History>();
            sequences.push(diffs.clone());
            end = sequences.len() - 1;
        }
        let mut diff = sequences[end][0];
        for sequence in sequences.iter().rev().skip(1) {
            diff = sequence[0] - diff;
        }
        values += diff;
    }

    values.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "114");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "2");
    }
}
