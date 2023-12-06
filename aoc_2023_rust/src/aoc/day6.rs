use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day 6 Solutions");
    println!("---------------");
    let input = read_input_for_day(6);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}
type Num = i64;
type Records = Vec<(Num, Num)>;

fn convert_str_to_values(string: &str) -> Vec<Num> {
    string
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .filter(|t| t.len() > 0)
        .map(|t| Num::from_str_radix(t, 10).unwrap())
        .collect::<Vec<Num>>()
}

fn convert_str_to_values_part2(string: &str) -> Vec<Num> {
    let value_string = string
        .split(':')
        .nth(1)
        .unwrap()
        .chars()
        .filter(|t| t.is_digit(10))
        .collect::<String>();

    vec![Num::from_str_radix(&value_string[..], 10).unwrap()]
}

fn parse_records_from(input: &String) -> Records {
    let time_str = input.split('\n').nth(0).unwrap();
    let distance_str = input.split('\n').nth(1).unwrap();
    let times = convert_str_to_values(time_str);
    let distances = convert_str_to_values(distance_str);

    assert_eq!(times.len(), distances.len());
    times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| (t, d))
        .collect::<Records>()
}

fn parse_records_part2_from(input: &String) -> Records {
    let time_str = input.split('\n').nth(0).unwrap();
    let distance_str = input.split('\n').nth(1).unwrap();
    let times = convert_str_to_values_part2(time_str);
    let distances = convert_str_to_values_part2(distance_str);

    assert_eq!(times.len(), distances.len());
    times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| (t, d))
        .collect::<Records>()
}

// Find the product of the number of ways that you can beat the record of each race
fn solve_part1(input: &String) -> String {
    let records = parse_records_from(input);
    let mut prod = 1;
    for (time, distance) in records {
        prod *= (0..time)
            .map(|t| if t * (time - t) > distance { 1 } else { 0 })
            .fold(0, |acc, win| acc + win)
    }

    prod.to_string()
}

fn solve_part2(input: &String) -> String {
    let records = parse_records_part2_from(input);
    let mut prod = 1;
    for (time, distance) in records {
        prod *= (0..time)
            .map(|t| if t * (time - t) > distance { 1 } else { 0 })
            .fold(0, |acc, win| acc + win)
    }

    prod.to_string()
}

#[cfg(test)]
mod test {

    use super::*;

    const INPUT: [&str; 1] = ["
Time:      7  15   30
Distance:  9  40  200
"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_records() {
        let records = parse_records_from(&get_input(0));
        let expected_records = vec![(7, 9), (15, 40), (30, 200)];
        assert_eq!(records.len(), expected_records.len());
        for (record, expected_record) in records.iter().zip(expected_records.iter()) {
            let (time, distance) = record;
            let (expected_time, expected_distance) = expected_record;
            assert_eq!(time, expected_time);
            assert_eq!(distance, expected_distance);
        }
    }

    #[test]
    fn test_parse_records_part2() {
        let records = parse_records_part2_from(&get_input(0));
        let expected_records = vec![(71530, 940200)];
        assert_eq!(records.len(), expected_records.len());
        for (record, expected_record) in records.iter().zip(expected_records.iter()) {
            let (time, distance) = record;
            let (expected_time, expected_distance) = expected_record;
            assert_eq!(time, expected_time);
            assert_eq!(distance, expected_distance);
        }
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "288");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "71503");
    }
}
