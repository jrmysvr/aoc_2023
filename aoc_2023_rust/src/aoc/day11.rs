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

fn find_empty_rows(universe: &Universe) -> HashSet<usize> {
    let mut empty_rows = HashSet::<usize>::new();
    for (r, row) in universe.split('\n').enumerate() {
        if HashSet::<char>::from_iter(row.chars()).len() == 1 {
            empty_rows.insert(r);
        }
    }
    empty_rows
}

type Galaxy = (usize, usize);
type Galaxies = Vec<Galaxy>;
fn find_galaxies_in_expanded(universe: &Universe, expansion: usize) -> Vec<Galaxy> {
    let empty_rows = find_empty_rows(universe);

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

    let empty_cols = find_empty_rows(&rotated_universe);

    let mut galaxies = Vec::<(usize, usize)>::new();
    let mut row_mod = 0;
    for (r, row) in universe.split('\n').enumerate() {
        let mut col_mod = 0;
        if empty_rows.contains(&r) {
            row_mod += expansion - 1;
            continue;
        }
        for (c, col) in row.chars().enumerate() {
            if empty_cols.contains(&c) {
                col_mod += expansion - 1;
                continue;
            }
            if col == '#' {
                galaxies.push((r + row_mod, c + col_mod));
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
    let universe = parse_universe_from(input);
    let galaxies = find_galaxies_in_expanded(&universe, 2);
    let pairs = calculate_pairs_of(&galaxies);
    pairs
        .iter()
        .map(|(a, b)| calc_distance_between(a, b))
        .fold(0, |acc, d| acc + d)
        .to_string()
}

fn solve_part2(input: &String) -> String {
    let universe = parse_universe_from(input);
    let galaxies = find_galaxies_in_expanded(&universe, 1_000_000);
    let pairs = calculate_pairs_of(&galaxies);
    pairs
        .iter()
        .map(|(a, b)| calc_distance_between(a, b))
        .fold(0, |acc, d| acc + d)
        .to_string()
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
    fn test_calc_distance() {
        let pairs = vec![
            (((6, 1), (11, 5)), 9),
            (((0, 4), (10, 9)), 15),
            (((2, 0), (7, 12)), 17),
            (((11, 0), (11, 5)), 5),
        ];
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
    fn test_full_part2() {
        // assert_eq!(solve_part2(&get_input(0)), "1030");
        // assert_eq!(solve_part2(&get_input(0)), "8410");
    }
}
