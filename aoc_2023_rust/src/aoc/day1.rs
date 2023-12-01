use crate::aoc::input::read_input_for_day;
use lazy_static::lazy_static;
use regex::Regex;

pub fn run() {
    println!("Day 1 Solutions");
    println!("---------------");
    let input = read_input_for_day(1);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

fn clean_input(input: &str) -> String {
    String::from(input)
        .trim()
        .split("\n")
        .map(|ipt| {
            ipt.chars()
                .filter(|i| !i.is_whitespace())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn parse_input(input: &String) -> Vec<String> {
    clean_input(input).split("\n").map(String::from).collect()
}

lazy_static! {
    static ref NUMBER_PATTERN: Regex =
        Regex::new(r"^(one|two|three|four|five|six|seven|eight|nine|[1-9])").unwrap();
}

fn num_str_to_byte(num_str: &str) -> u8 {
    let digit = match num_str {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Unknown number {num_str}"),
    };

    digit + 48
}

fn line_to_bytes(line: String) -> Vec<u8> {
    let mut bytes = Vec::<u8>::new();
    for i in 0..line.len() {
        if let Some(cap) = NUMBER_PATTERN.captures(&line[i..]) {
            bytes.push(num_str_to_byte(&cap[0]));
        }
    }
    bytes
}

fn calibration_value_from(line: String, digits_only: bool) -> i32 {
    let bytes = if digits_only {
        line.into_bytes()
    } else {
        line_to_bytes(line)
    };

    let digits: Vec<u8> = bytes
        .iter()
        .filter(|&&byte| byte >= 48 && byte <= 57)
        .map(|byte| byte - 48)
        .collect();
    let tens = digits[0];
    let ones = digits[digits.len() - 1];
    (tens * 10 + ones) as i32
}

fn get_all_calibration_values(lines: Vec<String>, digits_only: bool) -> Vec<i32> {
    lines
        .into_iter()
        .map(|line| calibration_value_from(line, digits_only))
        .collect()
}

fn solve_part1(input: &String) -> i32 {
    let input_lines = parse_input(&input);
    let calibration_values = get_all_calibration_values(input_lines, true);
    calibration_values.iter().fold(0, |acc, cal| acc + cal)
}

fn solve_part2(input: &String) -> i32 {
    let input_lines = parse_input(&input);
    let calibration_values = get_all_calibration_values(input_lines, false);
    calibration_values.iter().fold(0, |acc, cal| acc + cal)
}

#[cfg(test)]
mod test_part_1 {
    use crate::aoc::day1::*;

    const INPUT: &str = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    fn get_input() -> String {
        String::from(INPUT)
    }

    #[test]
    fn test_parse_input() {
        let parsed_input = parse_input(&get_input());
        assert_eq!(parsed_input.len(), 4);
        assert_eq!(parsed_input[0], String::from("1abc2"));
    }

    #[test]
    fn test_calibration_values() {
        assert_eq!(calibration_value_from(String::from("1234567890"), true), 10);
        assert_eq!(calibration_value_from(String::from("1abc2"), true), 12);
    }

    #[test]
    fn test_get_all_calibration_values() {
        let expected_values = vec![12, 38, 15, 77];
        let parsed_input = parse_input(&get_input());
        assert_eq!(
            get_all_calibration_values(parsed_input, true),
            expected_values
        );
    }

    #[test]
    fn test_full() {
        assert_eq!(142, solve_part1(&get_input()));
    }
}

#[cfg(test)]
mod test_part_2 {
    use crate::aoc::day1::*;

    const INPUT: &str = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    fn get_input() -> String {
        String::from(INPUT)
    }

    #[test]
    fn test_num_str_to_byte() {
        assert_eq!(num_str_to_byte("two"), 50);
        assert_eq!(num_str_to_byte("2"), 50);
        assert_eq!(num_str_to_byte("five"), 53);
    }

    #[test]
    fn test_line_to_bytes() {
        let line = String::from("two1nine");
        assert_eq!(line_to_bytes(line), vec![50, 49, 57]);
    }

    #[test]
    fn test_calibration_values() {
        assert_eq!(calibration_value_from(String::from("two1nine"), false), 29);
        assert_eq!(calibration_value_from(String::from("two1nine2"), false), 22);
        assert_eq!(
            calibration_value_from(String::from("eighttwothree"), false),
            83
        );
        assert_eq!(
            calibration_value_from(String::from("7abcsixteen"), false),
            76
        );
        assert_eq!(calibration_value_from(String::from("zero7"), false), 77);
        assert_eq!(calibration_value_from(String::from("01zero"), false), 11);
        assert_eq!(
            calibration_value_from(String::from("xtwone3four"), false),
            24
        );
        assert_eq!(
            calibration_value_from(String::from("seventgb4ninefive29twonegnb"), false),
            71
        );
    }

    #[test]
    fn test_get_all_calibration_values() {
        let expected_values = vec![29, 83, 13, 24, 42, 14, 76];
        let parsed_input = parse_input(&get_input());
        assert_eq!(
            get_all_calibration_values(parsed_input, false),
            expected_values
        );
    }

    #[test]
    fn test_full() {
        assert_eq!(281, solve_part2(&get_input()));
    }
}
