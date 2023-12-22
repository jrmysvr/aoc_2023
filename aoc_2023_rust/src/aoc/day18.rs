#![allow(dead_code)]
#![allow(unused_variables)]

use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day 18 Solutions");
    println!("---------------");
    let input = read_input_for_day(18);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type DigPlan = Vec<(char, usize)>;

fn parse_dig_plan_from(input: &String) -> DigPlan {
    input
        .split('\n')
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .map(|row| {
            (
                row[0].chars().nth(0).unwrap(),
                usize::from_str_radix(row[1], 10).unwrap(),
            )
        })
        .collect::<DigPlan>()
}

type Trench = Vec<(usize, usize)>;
fn dig_trench(plan: &DigPlan) -> Trench {
    let mut trench = Trench::new();
    let mut pos = (0isize, 0isize);
    for (direction, meters) in plan {
        let dig = match direction {
            'R' => (0, 1),
            'D' => (1, 0),
            'L' => (0, -1),
            'U' => (-1, 0),
            _ => panic!("Unsupported direction: {direction}"),
        };
        for _ in 0..*meters {
            pos = (dig.0 + pos.0, dig.1 + pos.1);
            trench.push((pos.0 as usize, pos.1 as usize));
        }
    }

    trench
}

fn calc_area_between(a: usize, b: usize) -> usize {
    b.checked_sub(a).unwrap_or(0).checked_sub(1).unwrap_or(0)
}

fn calc_volume_of(trench: &Trench) -> usize {
    let mut ix = 0;
    let mut volume = 0;
    loop {
        let mut holes = trench
            .iter()
            .filter(|dig| dig.0 == ix)
            .map(|d| *d)
            .collect::<Trench>();

        if holes.len() == 0 {
            break;
        }

        holes.sort_by(|a, b| a.1.cmp(&b.1));
        for hi in 0..(holes.len() - 1) {
            let curr_vol = calc_area_between(holes[hi].1, holes[hi + 1].1);
            volume += curr_vol;
        }
        volume += holes.len();
        ix += 1;
    }

    volume
}

fn visualize(trench: &Trench) {
    let max_c = trench.iter().fold(0, |acc, d| std::cmp::max(acc, d.1 + 1));
    let max_r = trench.iter().fold(0, |acc, d| std::cmp::max(acc, d.0 + 1));
    let mut grid = Vec::<Vec<char>>::new();
    for rx in 0..max_r {
        let mut row = Vec::<char>::new();
        for cx in 0..max_c {
            row.push('.');
        }
        grid.push(row);
    }
    for (r, c) in trench {
        if *r < max_r && *c < max_c {
            grid[*r][*c] = '#';
        }
    }

    for row in grid {
        for ch in row {
            print!("{}", ch);
        }
        println!("");
    }
}

fn solve_part1(input: &String) -> String {
    let dig_plan = parse_dig_plan_from(input);
    let trench = dig_trench(&dig_plan);
    // visualize(&trench);
    let volume = calc_volume_of(&trench);
    volume.to_string()
}

fn solve_part2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 2] = [
        "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        "
R 6 (#70c710)
D 1 (#0dc571)
L 1 (#5713f0)
D 1 (#0dc571)
R 1 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
    ];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_calc_length_of_trench() {
        let input = get_input(0);
        let dig_plan = parse_dig_plan_from(&input);
        let trench = dig_trench(&dig_plan);
        assert_eq!(trench.len(), 38);
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "62");
        assert_eq!(solve_part1(&get_input(1)), "62");
    }

    #[test]
    #[ignore]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
