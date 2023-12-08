#![allow(dead_code)]
#![allow(unused_variables)]

use crate::aoc::input::read_input_for_day;
use std::collections::HashMap;

pub fn run() {
    println!("Day 8 Solutions");
    println!("---------------");
    let input = read_input_for_day(8);
    let part1 = solve_part1(&input);
    println!("\tPart1: {part1}");
    let part2 = solve_part2(&input);
    println!("\tPart2: {part2}");
}

/* This is overkill, but maybe it would be fun to learn how to make an iterator
 * that will also cycle
#[derive(Debug, PartialEq)]
struct Instructions {
    raw: String,
    ix: usize,
}

impl Instructions {
    fn new(instr: &str) -> Self {
        Self {
            raw: String::from(instr),
            ix: 0,
        }
    }
}

impl Iterator for Instructions {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let nxt = self.raw.chars().nth(self.ix);
        self.ix += 1;
        nxt
    }
}
*/

type Instructions = String;

fn parse_instructions_from(input: &String) -> Instructions {
    let instruction_str = input.split('\n').nth(0).unwrap().trim();
    Instructions::from(instruction_str)
}

type Network<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse_network_from(input: &String) -> Network {
    let lines = input.split('\n').skip(2);
    let mut network = Network::new();
    for line in lines {
        let node_name = line.split(' ').nth(0).unwrap();
        let edge_str = line
            .split(" = ")
            .nth(1)
            .unwrap()
            .trim_start_matches("(")
            .trim_end_matches(")");
        let edges = edge_str.split(", ").collect::<Vec<&str>>();
        network.insert(node_name, (edges[0], edges[1]));
    }

    network
}

fn solve_part1(input: &String) -> String {
    let instructions = parse_instructions_from(input);
    let network = parse_network_from(input);

    let mut steps = 0;
    let end = "ZZZ";
    let mut node_name = "AAA";
    let mut it = instructions.chars().cycle();
    while node_name != end {
        steps += 1;
        let (left, right) = network[node_name];
        match it.next().unwrap() {
            'R' => node_name = right,
            'L' => node_name = left,
            instr => panic!("Invalid instruction: {instr}"),
        }
    }

    steps.to_string()
}

fn solve_part2(input: &String) -> String {
    let instructions = parse_instructions_from(input);
    let network = parse_network_from(input);

    let mut steps = 0;

    let keys = network.keys().clone();
    let mut node_names = keys
        .into_iter()
        .filter(|k| k.ends_with("A"))
        .map(|k| *k)
        .collect::<Vec<&str>>();

    steps.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 3] = [
        "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
",
        "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
    ];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_instructions() {
        let instructions = parse_instructions_from(&get_input(0));
        let expected_instructions = Instructions::from("RL");
        assert_eq!(instructions, expected_instructions);
    }

    #[test]
    fn test_cycle_instructions() {
        let instructions = parse_instructions_from(&get_input(0));
        let expected_instructions = "RLRLRL";
        let mut it = instructions.chars().cycle();
        for expected in expected_instructions.chars() {
            assert_eq!(it.next(), Some(expected));
        }
    }

    #[test]
    fn test_parse_network() {
        let input = &get_input(0);
        let network = parse_network_from(&input);
        let expected_network = Network::from([
            ("AAA", ("BBB", "CCC")),
            ("BBB", ("DDD", "EEE")),
            ("CCC", ("ZZZ", "GGG")),
            ("DDD", ("DDD", "DDD")),
            ("EEE", ("EEE", "EEE")),
            ("GGG", ("GGG", "GGG")),
            ("ZZZ", ("ZZZ", "ZZZ")),
        ]);
        assert_eq!(network, expected_network);
    }

    #[test]
    fn test_full_part1_0() {
        assert_eq!(solve_part1(&get_input(0)), "2");
    }

    #[test]
    fn test_full_part1_1() {
        assert_eq!(solve_part1(&get_input(1)), "6");
    }

    #[test]
    fn test_full_part2_2() {
        assert_eq!(solve_part2(&get_input(2)), "6");
    }
}
