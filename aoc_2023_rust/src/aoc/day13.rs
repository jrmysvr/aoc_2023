#![allow(dead_code)]
#![allow(unused_variables)]

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

type Pattern = Vec<String>;
type Patterns = Vec<Pattern>;

fn parse_patterns_from(input: &String) -> Patterns {
    input
        .split("\n\n")
        .map(|p| p.split('\n').map(|s| s.to_string()).collect::<Pattern>())
        .collect::<Patterns>()
}

fn rotate(pattern: &Pattern) -> Pattern {
    let n_cols = pattern[0].len();
    let mut rotated_pattern = Pattern::new();
    for c in 0..n_cols {
        let mut col = Vec::<char>::new();
        for row in pattern.iter().rev() {
            col.push(row.chars().nth(c).unwrap());
        }
        let rotated = col.iter().collect::<String>();
        rotated_pattern.push(rotated);
    }

    rotated_pattern
}

fn find_reflection_in_rows_of(pattern: &Pattern) -> (Option<usize>, Option<usize>) {
    // loop through all row indexs.
    //  - check two adjacent rows at each index
    //  - If the adjacent rows match, try to each subsequent row until either
    //    a mismatch is detected or the end of the pattern is reached.
    //    - If a mismatch is detected, continue trying the next row of the pattern
    //    - If the end of the pattern is reached, then the reflection index has been found
    let mut i = 0;
    while i < pattern.len() - 1 {
        if pattern[i] == pattern[i + 1] {
            let mut ix = 1;
            loop {
                if i.checked_sub(ix).is_none() || i + ix + 1 >= pattern.len() {
                    return (Some(i + 1), None);
                }
                if pattern[i - ix] != pattern[i + ix + 1] {
                    break;
                }
                ix += 1
            }
        }
        i += 1;
    }
    (None, None)
}

fn find_reflection_in(pattern: &Pattern) -> (Option<usize>, Option<usize>) {
    // Find first rows which are reflected
    let (r, c) = find_reflection_in_rows_of(pattern);
    if r.is_some() || c.is_some() {
        return (r, c);
    }
    // Find first reflected columns if reflected rows weren't found.
    let rotated = rotate(pattern);
    let (r, c) = find_reflection_in_rows_of(&rotated);
    if r.is_some() || c.is_some() {
        return (c, r);
    }

    (None, None)
}

fn find_reflection_in_modified(pattern: &Pattern) -> (Option<usize>, Option<usize>) {
    let orig_pattern = pattern.clone();
    let n_rows = pattern[0].len();
    let (mut r, mut c): (Option<usize>, Option<usize>) = (None, None);
    let mut row_reflection_found = false;
    for i in 0..orig_pattern.len() {
        for j in 0..n_rows {
            let mut pattern = orig_pattern.clone();
            let mut row = pattern[i].chars().collect::<Vec<char>>();
            let mut ch = row[j];
            ch = if ch == '.' { '#' } else { '.' };
            row[j] = ch;
            pattern[i] = row.iter().collect::<String>();


            // Find first rows which are reflected
            let (temp_r, temp_c) = find_reflection_in_rows_of(&pattern);
            if temp_r.is_some() || temp_c.is_some() {
                r = if temp_r.is_some() {
                    let orig_r = if r.is_some() { r.unwrap() } else { usize::MAX };
                    Some(std::cmp::min(orig_r, temp_r.unwrap()))
                } else {
                    None
                };
                c = if temp_c.is_some() {
                    let orig_c = if c.is_some() { c.unwrap() } else { usize::MAX };
                    Some(std::cmp::min(orig_c, temp_c.unwrap()))
                } else {
                    None
                };
                row_reflection_found = true;
            } else if ! row_reflection_found {
                // Find first reflected columns if reflected rows weren't found.
                let rotated = rotate(&pattern);
                let (temp_c, temp_r) = find_reflection_in_rows_of(&rotated);

                if temp_r.is_some() || temp_c.is_some() {
                    r = if temp_r.is_some() {
                        let orig_r = if r.is_some() { r.unwrap() } else { usize::MAX };
                        Some(std::cmp::min(orig_r, temp_r.unwrap()))
                    } else {
                        None
                    };
                    c = if temp_c.is_some() {
                        let orig_c = if c.is_some() { c.unwrap() } else { usize::MAX };
                        Some(std::cmp::min(orig_c, temp_c.unwrap()))
                    } else {
                        None
                    };
                }
            }
        }
    }

    (r, c)
}

// Calculate a "summary" value based on the number of reflected columns and rows
// in the input patterns
fn solve_part1(input: &String) -> String {
    let patterns = parse_patterns_from(input);

    let mut col_count = 0;
    let mut row_count = 0;
    for pattern in patterns.iter() {
        let (r, c) = find_reflection_in(pattern);
        col_count += if let Some(c_count) = c { c_count } else { 0 };
        row_count += if let Some(r_count) = r { r_count } else { 0 };
    }

    (col_count + 100 * row_count).to_string()
}

fn solve_part2(input: &String) -> String {
    let patterns = parse_patterns_from(input);

    let mut col_count = 0;
    let mut row_count = 0;
    for pattern in patterns.iter() {
        let (r, c) = find_reflection_in_modified(pattern);
        col_count += if let Some(c_count) = c { c_count } else { 0 };
        row_count += if let Some(r_count) = r { r_count } else { 0 };
    }

    (col_count + 100 * row_count).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
        "];

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
    fn test_rotate() {
        let pattern = vec!["#.##.".to_string(), "#..##".to_string()];
        let expected_rotated = vec![
            "##".to_string(),
            "..".to_string(),
            ".#".to_string(),
            "##".to_string(),
            "#.".to_string(),
        ];
        let actual_rotated = rotate(&pattern);

        assert_eq!(actual_rotated, expected_rotated);
        let pattern = vec![
            "#.##.".to_string(),
            "#..##".to_string(),
            "#..#.".to_string(),
        ];
        let expected_rotated = vec![
            "###".to_string(),
            "...".to_string(),
            "..#".to_string(),
            "###".to_string(),
            ".#.".to_string(),
        ];
        let actual_rotated = rotate(&pattern);
        assert_eq!(actual_rotated, expected_rotated);

        let pattern = vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
        ];
        let expected_rotated = vec![
            "#..##.#".to_string(),
            "...##..".to_string(),
            "###..##".to_string(),
            ".#....#".to_string(),
            "#.#..#.".to_string(),
            "#.#..#.".to_string(),
            ".#....#".to_string(),
            "###..##".to_string(),
            "...##..".to_string(),
        ];
        let actual_rotated = rotate(&pattern);
        assert_eq!(actual_rotated, expected_rotated);
    }

    #[test]
    fn test_find_reflection() {
        let pattern = vec![
            "##......#".to_string(),
            "##......#".to_string(),
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
        ];
        let (actual_r, actual_c) = find_reflection_in(&pattern);
        assert_eq!(actual_r, Some(1));
        assert_eq!(actual_c, None);
        let input = get_input(0);
        let patterns = parse_patterns_from(&input);
        let expected_reflections = vec![(None, Some(5)), (Some(4), None)];
        for (pattern, expected) in patterns.iter().zip(expected_reflections.iter()) {
            let (actual_r, actual_c) = find_reflection_in(pattern);
            let (expected_r, expected_c) = expected;
            assert_eq!(actual_r, *expected_r);
            assert_eq!(actual_c, *expected_c);
        }
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "405");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "400");
    }
}
