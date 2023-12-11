#![allow(dead_code)]
#![allow(unused_variables)]

use crate::aoc::input::read_input_for_day;
use std::collections::HashSet;

pub fn run() {
    println!("Day 11 Solutions");
    println!("---------------");
    let input = read_input_for_day(11);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Universe = String;
fn parse_universe_from(input: &String) -> Universe {
    input.clone()
}

fn expand_universe(universe: &Universe) -> Universe {
    // find empty rows
    let mut empty_rows = Vec::<usize>::new();
    for (r, row) in universe.split('\n').enumerate() {
        if HashSet::<char>::from_iter(row.chars()).len() == 1 {
            empty_rows.push(r);
        }
    }

    let ncols = universe.split('\n').nth(0).unwrap().len();
    let rotated_universe = (0..ncols)
        .map(|c| {
            universe
                .split('\n')
                .map(|r| r.chars().nth(c).unwrap())
                .collect::<String>()
        })
        .map(|col| col + "\n")
        .collect::<String>();

    // find empty cols
    let mut empty_cols = Vec::<usize>::new();
    for (c, col) in rotated_universe.split('\n').enumerate() {
        if HashSet::<char>::from_iter(col.chars()).len() == 1 {
            empty_cols.push(c);
        }
    }

    // Expand the universe
    let mut new_universe = Universe::new();
    for (r, row) in universe.split('\n').enumerate() {
        new_universe += row;
        new_universe += "\n";
        if empty_rows.contains(&r) {
            new_universe += row;
            new_universe += "\n";
        }
    }
    let new_rotated = (0..ncols)
        .map(|c| {
            new_universe
                .trim()
                .split('\n')
                .map(|r| r.chars().nth(c).unwrap())
                .collect::<String>()
        })
        .map(|col| col + "\n")
        .collect::<String>();
    let mut rotated_expanded = String::new();
    for (c, col) in new_rotated.split('\n').enumerate() {
        rotated_expanded += col;
        rotated_expanded += "\n";
        if empty_cols.contains(&c) {
            rotated_expanded += col;
            rotated_expanded += "\n";
        }
    }

    let nrows = rotated_expanded.split('\n').nth(0).unwrap().len();
    let expanded_universe = (0..nrows)
        .map(|r| {
            rotated_expanded
                .trim()
                .split('\n')
                .map(|c| c.chars().nth(r).unwrap())
                .collect::<String>()
        })
        .map(|row| row + "\n")
        .collect::<String>();
    expanded_universe.trim().to_string()
}

type Galaxy = (usize, usize);
type Galaxies = Vec<Galaxy>;
fn find_galaxies_in(universe: &Universe) -> Vec<Galaxy> {
    let mut galaxies = Vec::<(usize, usize)>::new();
    for (r, row) in universe.split('\n').enumerate() {
        for (c, col) in row.chars().enumerate() {
            if col == '#' {
                galaxies.push((r, c));
            }
        }
    }

    galaxies
}

fn calculate_pairs_of(galaxies: &Galaxies) -> Vec<(Galaxy, Galaxy)> {
    let mut pairs = Vec::<(Galaxy, Galaxy)>::new();
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            pairs.push((galaxies[i], galaxies[j]));
        }
    }

    pairs
}

fn calc_distance_between(a: &Galaxy, b: &Galaxy) -> usize {
    let (ar, ac) = a;
    let (br, bc) = b;

    let mx_r = if ar < br { br } else { ar };
    let mn_r = if ar < br { ar } else { br };

    let mx_c = if ac < bc { bc } else { ac };
    let mn_c = if ac < bc { ac } else { bc };

    let r = mx_r.checked_sub(*mn_r).unwrap();
    let c = mx_c.checked_sub(*mn_c).unwrap();

    r + c
}

fn solve_part1(input: &String) -> String {
    // TODO:
    let universe = parse_universe_from(input);
    let expanded = expand_universe(&universe);
    let galaxies = find_galaxies_in(&expanded);
    let pairs = calculate_pairs_of(&galaxies);
    // Calculate sum of distances
    pairs
        .iter()
        .map(|(a, b)| calc_distance_between(a, b))
        .fold(0, |acc, d| acc + d)
        .to_string()
}

fn solve_part2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_expand_universe() {
        let universe = parse_universe_from(&get_input(0));
        let expanded = expand_universe(&universe);
        let expected_universe_str = "
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."
            .trim()
            .to_string();
        assert_eq!(expanded.to_string(), expected_universe_str);
    }

    #[test]
    fn test_find_galaxies() {
        let universe = parse_universe_from(&get_input(0));
        let expanded = expand_universe(&universe);
        let galaxies = find_galaxies_in(&expanded);
        let expected_galaxies = vec![
            (0, 4),
            (1, 9),
            (2, 0),
            (5, 8),
            (6, 1),
            (7, 12),
            (10, 9),
            (11, 0),
            (11, 5),
        ];
        for (actual, expected) in galaxies.iter().zip(expected_galaxies.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_pair_galaxies() {
        let universe = parse_universe_from(&get_input(0));
        let expanded = expand_universe(&universe);
        let galaxies = find_galaxies_in(&expanded);
        let pairs = calculate_pairs_of(&galaxies);
        assert_eq!(pairs.len(), 36);
        let expected_pairs = vec![
            // Paired with 1
            ((0, 4), (1, 9)),
            ((0, 4), (2, 0)),
            ((0, 4), (5, 8)),
            ((0, 4), (6, 1)),
            ((0, 4), (7, 12)),
            ((0, 4), (10, 9)),
            ((0, 4), (11, 0)),
            ((0, 4), (11, 5)),
            // Paired with 2
            ((1, 9), (2, 0)),
            ((1, 9), (5, 8)),
        ];

        for (actual, expected) in pairs.iter().zip(expected_pairs.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_calc_distance() {
        let pairs = vec![
            (((6, 1), (11, 5)), 9),
            (((0, 4), (10, 9)), 15),
            (((2, 0), (7, 12)), 17),
            (((11, 0), (11, 5)), 5),
        ];
        // for ((a, b), expected) in pairs.iter().zip(expected_distances.iter()) {
        for ((a, b), expected) in pairs {
            let distance = calc_distance_between(&a, &b);
            assert_eq!(distance, expected);
        }
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "374");
    }

    #[test]
    #[ignore]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
