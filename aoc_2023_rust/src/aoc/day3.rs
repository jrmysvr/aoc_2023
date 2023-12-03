#![allow(dead_code)]
#![allow(unused_variables)]

use crate::aoc::input::read_input_for_day;
use lazy_static::lazy_static;
use std::collections::HashSet;

pub fn run() {
    println!("Day 3 Solutions");
    println!("---------------");
    let input = read_input_for_day(3);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

lazy_static! {
    static ref SYMBOLS: HashSet<char> =
        HashSet::from_iter("!@#$%^&*?,;\"':~`+=_-<>{}[]|/\\".chars());
}

type Coordinate = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cell {
    row: usize,
    col_rng: (usize, usize),
    val: String,
}

impl Cell {
    fn new(val: String, coor: Coordinate) -> Self {
        let (r, c) = coor;
        let len = val.len();
        Self {
            val: val,
            row: r,
            col_rng: (c, c + len),
        }
    }

    fn is_symbol(&self) -> bool {
        !self.is_number()
    }

    fn is_number(&self) -> bool {
        self.val.chars().next().unwrap().is_digit(10)
    }

    fn is_neighbor_of(&self, cell: &Cell) -> bool {
        for c in cell.col_rng.0..cell.col_rng.1 {
            if self.is_neighbor_of_coor((cell.row, c)) {
                return true;
            }
        }

        false
    }

    fn is_neighbor_of_coor(&self, coor: Coordinate) -> bool {
        let (r, c) = coor;
        Vec::<(isize, isize)>::from([
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ])
        .iter()
        .map(|(rx, cx)| {
            (
                r.checked_add_signed(*rx).unwrap_or(0),
                c.checked_add_signed(*cx).unwrap_or(0),
            )
        })
        // check if self.coor is in neighbors
        .filter(|(rx, cx)| *rx == self.row && self.col_rng.0 <= *cx && self.col_rng.1 > *cx)
        .peekable()
        .peek()
        .is_some()
    }

    fn num_val(&self) -> i32 {
        i32::from_str_radix(&self.val[..], 10).unwrap_or(-1)
    }
}

fn convert_input_to_cells(input: &String) -> Vec<Cell> {
    let mut cells = Vec::new();
    for (r, line) in input.split('\n').enumerate() {
        let mut num_str = String::from("");
        let mut num_row = 0;
        for (c, chr) in line.chars().enumerate() {
            if SYMBOLS.contains(&chr) {
                cells.push(Cell::new(chr.to_string(), (r, c)));
            } else if chr == '.' {
                if num_str.len() > 0 {
                    cells.push(Cell::new(num_str.clone(), (num_row, c - num_str.len())));
                    num_str = String::from("");
                    num_row = 0;
                }
            } else {
                // It's a number
                num_row = r;
                num_str.push(chr);
            }
        }
    }

    cells
}

// Find all numbers adjacent to symbols (including diagonally!).
fn solve_part1(input: &String) -> String {
    // Preprocess:
    //  - Parse input into `Cell`s
    // Find all symbol coordinates (r, c)
    // For each symbol coordinate, find all neighboring numbers
    // Sum all numbers found at these coordinates

    let cells = convert_input_to_cells(input);
    let symbol_cells = cells
        .iter()
        .filter(|cell| cell.is_symbol())
        .collect::<Vec<&Cell>>();
    let mut sum = 0;
    let mut number_cells = Vec::<&Cell>::new();
    for symbol_cell in symbol_cells.iter() {
        for cell in cells.iter() {
            if cell.is_number() && cell.is_neighbor_of(symbol_cell) {
                sum += cell.num_val();
                number_cells.push(cell);
            }
        }
    }
    let len = number_cells.len();
    let number_set: HashSet<&Cell> = HashSet::from_iter(number_cells.into_iter());
    assert!(number_set.len() == len);


    sum.to_string()
}

fn solve_part2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {

    use super::*;

    const INPUT0: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    const INPUT1: &str = "
467..114..
...*......
@.35..633.
......#...
617*...%..
.....+.58.
..592.....
~1....755.
...$.*....
.664.598..
";

    fn get_input0() -> String {
        String::from(INPUT0.trim())
    }

    fn get_input1() -> String {
        String::from(INPUT1.trim())
    }

    #[test]
    fn test_cells() {
        let number_cell = Cell::new("467".to_string(), (0, 0));
        assert_eq!(number_cell.num_val(), 467);
        assert!(number_cell.is_number());
        assert!(number_cell.is_neighbor_of_coor((0, 3)));
        assert!(number_cell.is_neighbor_of_coor((1, 3)));
        assert!(!number_cell.is_neighbor_of_coor((3, 3)));

        let symbol_cell = Cell::new("*".to_string(), (1, 3));
        assert!(symbol_cell.is_symbol());
        assert!(symbol_cell.is_neighbor_of_coor((0, 3)));
        assert!(symbol_cell.is_neighbor_of_coor((1, 4)));
        assert!(!symbol_cell.is_neighbor_of_coor((5, 5)));

        let neighbor_cell = Cell::new("*".to_string(), (1, 1));
        assert!(number_cell.is_neighbor_of(&neighbor_cell));
    }

    #[test]
    fn test_convert_input_to_cells() {
        let input = String::from("467..*");
        let expected_cells = vec![
            Cell::new("467".to_string(), (0, 0)),
            Cell::new("*".to_string(), (0, 5)),
        ];
        let cells = convert_input_to_cells(&input);
        assert_eq!(cells.len(), expected_cells.len());
        for (a, b) in cells.iter().zip(expected_cells.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_full_part1_0() {
        assert_eq!(solve_part1(&get_input0()), "4361");
    }

    #[test]
    fn test_full_part1_1() {
        assert_eq!(solve_part1(&get_input1()), "4420");
    }
}
