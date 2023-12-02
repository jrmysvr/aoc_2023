use crate::aoc::input::read_input_for_day;
use std::collections::HashMap;

pub fn run() {
    println!("Day 2 Solutions");
    println!("---------------");
    let input = read_input_for_day(2);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

// Find the sum of Game IDs whose cube counts could have come from
// a bag with 12 red cubes, 13 green cubes, and 14 blue cubes.
fn solve_part1(input: &String) -> String {
    let mut result = 0;
    let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    for (id_, line) in input.trim().split('\n').enumerate() {
        // IDs are 1 indexed
        let id = id_ + 1;
        let game = line.trim().split(':').nth(1).unwrap();
        let mut possible = true;
        for grab in game.trim().split(';') {
            for blocks in grab.trim().split(',') {
                match &blocks.trim().split(' ').take(2).collect::<Vec<&str>>()[..] {
                    [count, color] => {
                        if let Ok(count) = i32::from_str_radix(count, 10) {
                            if count > bag[color] {
                                possible = false;
                                break;
                            }
                        } else {
                            panic!("Count couldn't be parsed! {count}");
                        }
                    }
                    _ => panic!("Oops!: {blocks}"),
                }
            }
        }
        if possible {
            result += id
        };
    }

    result.to_string()
}

// Find the minimum count of blocks needed for each game, then calculate
// the sum of the products of each color count in each game.
fn solve_part2(input: &String) -> String {
    let mut result = 0;
    for line in input.trim().split('\n') {
        let game = line.trim().split(':').nth(1).unwrap();
        let mut color_count = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for grab in game.trim().split(';') {
            for blocks in grab.trim().split(',') {
                match &blocks.trim().split(' ').take(2).collect::<Vec<&str>>()[..] {
                    [count, color] => {
                        let count = i32::from_str_radix(count, 10).ok().unwrap();
                        color_count
                            .entry(color)
                            .and_modify(|c| *c = std::cmp::max(*c, count));
                    }
                    _ => panic!("Oops!: {blocks}"),
                }
            }
        }
        let power = color_count.values().fold(1, |acc, c| acc * c);
        result += power;
    }

    result.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_full_part1() {
        let input = String::from(INPUT);
        let result = solve_part1(&input);
        assert_eq!(result, String::from("8"));
    }

    #[test]
    fn test_full_part2() {
        let input = String::from(INPUT);
        let result = solve_part2(&input);
        assert_eq!(result, String::from("2286"));
    }
}
