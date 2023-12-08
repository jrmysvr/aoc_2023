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

type Num = i64;
type Instructions = String;
type Network<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse_instructions_from(input: &String) -> Instructions {
    let instruction_str = input.split('\n').nth(0).unwrap().trim();
    Instructions::from(instruction_str)
}

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

fn gcd(a: Num, b: Num) -> Num {
    let mut mx = if a > b { a } else { b };
    let mut mn = if a < b { a } else { b };

    let mut rem = mx % mn;
    while rem != 0 {
        mx = mn;
        mn = rem;
        rem = mx % mn;
    }

    mn
}

fn lcm(a: Num, b: Num) -> Num {
    a * b / gcd(a, b)
}

fn lcms(values: &Vec<Num>) -> Num {
    let mut output = lcm(values[0], values[1]);
    for i in 2..values.len() {
        output = lcm(values[i], output);
    }

    output
}

fn solve_part2(input: &String) -> String {
    let instructions = parse_instructions_from(input);
    let network = parse_network_from(input);

    let node_names = network
        .keys()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<&&str>>();
    let mut steps = vec![0; node_names.len()];
    for (i, name) in node_names.into_iter().enumerate() {
        let mut it = instructions.chars().cycle();
        let mut node = name;
        while !node.ends_with("Z") {
            steps[i] += 1;
            node = if it.next().unwrap() == 'L' {
                &network[node].0
            } else {
                &network[node].1
            };
        }
    }

    lcms(&steps).to_string()
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
    fn test_lcm() {
        assert_eq!(lcm(3, 4), 12);
        assert_eq!(lcm(4, 5), 20);
        let nums = vec![2, 3, 4, 5, 7];
        assert_eq!(lcms(&nums), 420);
    }

    #[test]
    fn test_parse_instructions() {
        let instructions = parse_instructions_from(&get_input(0));
        let expected_instructions = Instructions::from("RL");
        assert_eq!(instructions, expected_instructions);
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
