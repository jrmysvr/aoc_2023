use crate::aoc::input::read_input_for_day;

pub fn run() {
    println!("Day 3 Solutions");
    println!("---------------");
    let input = read_input_for_day(3);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
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

    fn is_gear(&self) -> bool {
        self.val == "*".to_string()
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
        for (c, chr) in line.chars().enumerate() {
            if !chr.is_digit(10) {
                if num_str.len() > 0 {
                    cells.push(Cell::new(num_str.clone(), (r, c - num_str.len())));
                    num_str = String::from("");
                }
                if chr != '.' {
                    cells.push(Cell::new(chr.to_string(), (r, c)));
                }
            } else {
                // It's a number
                num_str.push(chr);
            }
        }
        if num_str.len() > 0 {
            let c = line.len() - num_str.len();
            cells.push(Cell::new(num_str.clone(), (r, c)));
        }
    }

    cells
}

// Find the sum of all numbers adjacent to symbols (including diagonally!).
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
    for symbol_cell in symbol_cells.iter() {
        for cell in cells.iter() {
            if cell.is_number() && cell.is_neighbor_of(symbol_cell) {
                sum += cell.num_val();
            }
        }
    }
    sum.to_string()
}

// Find the sum of the products of numbers adjacent to all gears ("*") for
// gears with exactly two adjacent numbers.
fn solve_part2(input: &String) -> String {
    let cells = convert_input_to_cells(input);
    let gear_cells = cells
        .iter()
        .filter(|cell| cell.is_gear())
        .collect::<Vec<&Cell>>();
    let mut sum = 0;
    for gear_cell in gear_cells.iter() {
        let mut number_cells = Vec::<&Cell>::new();
        for cell in cells.iter() {
            if cell.is_number() && cell.is_neighbor_of(gear_cell) {
                number_cells.push(cell);
            }
            if number_cells.len() > 2 {
                break;
            }
        }

        if number_cells.len() == 2 {
            sum += number_cells.iter().fold(1, |acc, c| acc * c.num_val());
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod test {

    use super::*;

    const INPUT: [&str; 3] = [
        "
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
",
        "
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
",
        "
.......12.......935............184.720...243........589.652..........435.......
......*.....968*.....$............*........=..348...*..........986....*........
....291............612....290..........903........699......218*.......376......
..............156......$..*...891.&731....%..89...................523..........
................*...189..591.*................*.......783.....107..-...54.287..
...229*952.....938............470.555.......746...28.....+...*.................
...................................*...............@.........867.-....102..845.
563.727.....282....237..171.......892...183.......................989....*.....
....#..........+...*.......*..........................&......129+......491.....
.................53.....781...&295....@773.336......547........................
",
    ];

    fn get_input(ix: usize) -> String {
        INPUT[ix].trim().to_string()
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

        let input = String::from("229*952");
        let expected_cells = vec![
            Cell::new("229".to_string(), (0, 0)),
            Cell::new("*".to_string(), (0, 3)),
            Cell::new("952".to_string(), (0, 4)),
        ];
        let cells = convert_input_to_cells(&input);
        assert_eq!(cells.len(), expected_cells.len());
        for (actual, expected) in cells.iter().zip(expected_cells.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_full_part1_0() {
        assert_eq!(solve_part1(&get_input(0)), "4361");
    }

    #[test]
    fn test_full_part1_1() {
        assert_eq!(solve_part1(&get_input(1)), "4420");
    }

    #[test]
    fn test_full_part1_2() {
        assert_eq!(solve_part1(&get_input(2)), "23775");
    }

    #[test]
    fn test_full_part2_0() {
        assert_eq!(solve_part2(&get_input(0)), "467835");
    }
}
