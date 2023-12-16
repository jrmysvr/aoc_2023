#![allow(dead_code)]
#![allow(unused_variables)]

use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day 14 Solutions");
    println!("---------------");
    let input = read_input_for_day(14);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

fn calc_load_on(platform: &Vec<Vec<char>>) -> i32 {
    (0..platform.len())
        .map(|i| platform[i] 
            .iter()
            .filter(|&p| *p == 'O')
            .fold(0, |acc, _| acc + platform.len() - i))
        .fold(0, |acc, p| acc + p) as i32
}

fn solve_part1(input: &String) -> String {
    let mut platform = input
        .split('\n')
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let n_cols = platform[0].len();
    let mut rolling = true;
    while rolling {
        rolling = false;
        for i in 1..platform.len() {
            for j in 0..n_cols {
                if platform[i - 1][j] == '.' && platform[i][j] == 'O' {
                    platform[i - 1][j] = platform[i][j];
                    platform[i][j] = '.';
                    rolling = true;
                }
            }
        }
    }

    let load = calc_load_on(&platform);

    load.to_string()
}

fn solve_part2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "136");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
