#![allow(dead_code)]
#![allow(unused_variables)]

use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day 15 Solutions");
    println!("---------------");
    let input = read_input_for_day(15);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

fn hash(s: &str) -> u32 {
    let mut value = 0;
    for ch in s.chars() {
        value += ch as u32;
        value *= 17;
        value %= 256;
    }

    value
}

fn solve_part1(input: &String) -> String {
    input
        .chars()
        .filter(|&ch| ch != '\n')
        .collect::<String>()
        .split(',')
        .map(hash)
        .fold(0, |acc, h| acc + h)
        .to_string()
}

fn solve_part2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "1320");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
