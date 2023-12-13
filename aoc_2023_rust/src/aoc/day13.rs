use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day 13 Solutions");
    println!("---------------");
    let input = read_input_for_day(13);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Patterns<'a> = Vec<Vec<&'a str>>;

fn parse_patterns_from(input: &String) -> Patterns {
    let mut patterns = Vec::<Vec<&str>>::new();
    input
        .split("\n\n")
        .map(|p| p.split('\n').collect::<Vec<&str>>())
        .collect::<Patterns>()
}

fn rotate<'a>(patterns: &'a Patterns) -> Patterns<'a> {
    vec![]
}

fn solve_part1(input: &String) -> String {
    let patterns = parse_patterns_from(input);
    let rotated = rotate(&patterns);

    let mut col_count = 0;
    let mut row_count = 0;
    //check rows
    for pattern in patterns {
        for i in 0..pattern.len() {
        }
    }
    String::new()
}

fn solve_part2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = [""];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_equal_patterns() {
        let pattern_a = vec!["#.##.", "#..#."];

        let pattern_b = vec!["#.##.", "#..#."];

        assert_eq!(pattern_a, pattern_b);
        assert_eq!(pattern_a[..1], pattern_b[..1]);
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
