#![allow(dead_code)]
#![allow(unused_variables)]

use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day 12 Solutions");
    println!("---------------");
    let input = read_input_for_day(12);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Record = (String, Vec<usize>);
type Records = Vec<Record>;
fn parse_records_from(input: &String) -> Records {
    let mut records = Records::new();
    for line in input.split('\n') {
        let springs = line.split(' ').nth(0).unwrap();
        let groups = line
            .split(' ')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|g| usize::from_str_radix(g, 10).unwrap())
            .collect::<Vec<usize>>();
        records.push((springs.to_string(), groups));
    }

    records
}

fn count_arrangements(record: &Record) -> usize {
    let (springs, groups) = record;
    // TODO:
    // -Remove broken springs which are accounted for in the groups
    //  ???.### 1,1,3 => ???. 1,1
    // -Replace ? with . where "known" (between other # for instance)
    0
}

fn solve_part1(input: &String) -> String {
    let records = parse_records_from(input);
    records
        .iter()
        .map(count_arrangements)
        .fold(0, |acc, count| acc + count)
        .to_string()
}

fn solve_part2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_records() {
        let input = get_input(0);
        let records = parse_records_from(&input);
        let (springs, groups) = &records[0];
        assert_eq!(springs, "???.###");
        assert_eq!(*groups, vec![1, 1, 3]);
    }

    #[test]
    fn test_count_arrangements() {
        let input = get_input(0);
        let records = parse_records_from(&input);
        let expected_counts = vec![1, 4, 1, 1, 4, 10];
        for (record, expected_count) in records.iter().zip(expected_counts.iter()) {
            let count = count_arrangements(record);
            assert_eq!(count, *expected_count);
        }
    }

    #[test]
    #[ignore]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "21");
    }

    #[test]
    #[ignore]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
