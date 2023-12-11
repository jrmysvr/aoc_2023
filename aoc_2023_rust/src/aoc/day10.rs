#![allow(dead_code)]
#![allow(unused_variables)]

use crate::aoc::input::read_input_for_day;

use std::collections::HashSet;

pub fn run() {
    println!("Day 10 Solutions");
    println!("---------------");
    let input = read_input_for_day(10);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

type Coordinate = (usize, usize);
type Coordinates = HashSet<Coordinate>;

fn find_starting_point(input: &String) -> Coordinate {
    let nrows = input.len();
    let ncols = input.split('\n').nth(0).unwrap().len();
    for (r, line) in input.split('\n').enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == 'S' {
                return (r, c);
            }
        }
    }

    panic!("No starting point found!");
}

fn get_pipe_at(coor: &Coordinate, input: &String) -> char {
    let (r, c) = coor;
    if let Some(line) = input.split('\n').nth(*r) {
        if let Some(ch) = line.chars().nth(*c) {
            return ch;
        }
    }

    ' '
}

fn can_connect(a: &Coordinate, b: &Coordinate, input: &String) -> bool {
    let mut connections_found = Vec::<bool>::new();
    for (coor_a, coor_b) in vec![(a, b), (b, a)] {
        let (ra, ca) = coor_a;
        let connections = match get_pipe_at(coor_a, input) {
            '|' => vec![
                (ra.checked_sub(1).unwrap_or(*ra), *ca),
                (ra.checked_add(1).unwrap_or(*ra), *ca),
            ],
            '-' => vec![
                (*ra, ca.checked_sub(1).unwrap_or(*ca)),
                (*ra, ca.checked_add(1).unwrap_or(*ca)),
            ],
            'L' => vec![
                (*ra, ca.checked_add(1).unwrap_or(*ca)),
                (ra.checked_sub(1).unwrap_or(*ra), *ca),
            ],
            'J' => vec![
                (*ra, ca.checked_sub(1).unwrap_or(*ca)),
                (ra.checked_sub(1).unwrap_or(*ra), *ca),
            ],
            '7' => vec![
                (*ra, ca.checked_sub(1).unwrap_or(*ca)),
                (ra.checked_add(1).unwrap_or(*ra), *ca),
            ],
            'F' => vec![
                (*ra, ca.checked_add(1).unwrap_or(*ca)),
                (ra.checked_add(1).unwrap_or(*ra), *ca),
            ],
            'S' => return get_pipe_at(coor_b, input) != '.',
            _ => vec![],
        };

        for connection in connections {
            if *coor_b == connection {
                connections_found.push(true);
            }
        }
    }
    connections_found.len() > 1
}

fn find_next_connection_of(
    coor: &Coordinate,
    prev: &Coordinate,
    input: &String,
) -> Option<Coordinate> {
    let connections = find_connections_to(coor, input);
    let next_connections = connections
        .iter()
        .filter(|conn| *conn != prev)
        .map(|conn| *conn)
        .collect::<Coordinates>();

    next_connections.into_iter().next()
}

fn find_connections_to(coor: &Coordinate, input: &String) -> Coordinates {
    let mut connections = Coordinates::new();

    let (r, c) = coor;
    for rx in vec![-1, 0, 1] {
        for cx in vec![-1, 0, 1] {
            if rx == cx {
                continue;
            }
            let connection = (
                r.checked_add_signed(rx).unwrap_or(*r),
                c.checked_add_signed(cx).unwrap_or(*c),
            );
            if can_connect(coor, &connection, input) {
                connections.insert(connection);
            }
        }
    }

    connections
}

fn solve_part1(input: &String) -> String {
    let start = find_starting_point(input);
    let start_connections = find_connections_to(&start, input);
    let mut steps = 1;
    for conn in start_connections {
        let mut step = 1;
        let mut next = conn;
        let mut prev = start.clone();
        while let Some(next_next) = find_next_connection_of(&next, &prev, input) {
            prev = next;
            next = next_next;
            step += 1;
            if next == start {
                break;
            }
        }
        steps = std::cmp::max(steps, step);
    }
    if steps % 2 == 0 {
        (steps / 2).to_string()
    } else {
        ((steps / 2) + 1).to_string()
    }
}

fn solve_part2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 3] = [
        "
.....
.S-7.
.|.|.
.L-J.
.....
",
        /*
                "
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
        ",
        */
        "
.FF7.
.FJ|.
SJ.L7
|F--J
LJ...
",
        "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
    ];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_find_starting_point() {
        let input = get_input(0);
        let start_coor = find_starting_point(&input);
        assert_eq!(start_coor, (1, 1));

        let input = get_input(1);
        let start_coor = find_starting_point(&input);
        assert_eq!(start_coor, (2, 0));
    }

    #[test]
    fn test_find_connections() {
        let input = get_input(0);
        let start = find_starting_point(&input);
        let coors = find_connections_to(&start, &input);
        let expected_coors = Coordinates::from_iter([(1, 2), (2, 1)].into_iter());

        assert_eq!(coors, expected_coors);
    }

    #[test]
    fn test_can_connect_0() {
        let input = get_input(0);
        let connections = vec![
            ((1, 1), (1, 2)),
            ((1, 1), (2, 1)),
            ((3, 3), (2, 3)),
            ((3, 3), (3, 2)),
        ];

        for (a, b) in connections.iter() {
            assert!(
                can_connect(a, b, &input),
                "{a:?} doesn't connect to {b:?} but it should!"
            );
        }

        let not_connections = vec![
            ((1, 1), (1, 0)),
            ((1, 1), (0, 1)),
            ((3, 3), (3, 4)),
            ((3, 3), (4, 3)),
        ];

        for (a, b) in not_connections.iter() {
            assert!(
                !can_connect(a, b, &input),
                "{a:?} connects to {b:?}, but it shouldn't!"
            );
        }
    }

    #[test]
    fn test_can_connect_1() {
        let input = get_input(1);
        let connections = vec![((0, 2), (0, 3)), ((0, 3), (1, 3))];

        for (a, b) in connections.iter() {
            assert!(
                can_connect(a, b, &input),
                "{a:?} doesn't connect to {b:?} but it should!"
            );
        }

        let not_connections = vec![
            ((0, 1), (0, 0)),
            ((0, 1), (0, 2)),
            ((0, 3), (0, 4)),
            ((1, 3), (1, 4)),
        ];

        for (a, b) in not_connections.iter() {
            assert!(
                !can_connect(a, b, &input),
                "{a:?} connects to {b:?}, but it shouldn't!"
            );
        }
    }

    #[test]
    fn test_full_part1_0() {
        assert_eq!(solve_part1(&get_input(0)), "4");
    }

    #[test]
    fn test_full_part1_1() {
        assert_eq!(solve_part1(&get_input(1)), "8");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
