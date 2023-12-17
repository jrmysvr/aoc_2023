#![allow(dead_code)]
#![allow(unused_variables)]

use crate::aoc::input::read_input_for_day;
use std::collections::HashSet;

pub fn run() {
    println!("Day 16 Solutions");
    println!("---------------");
    let input = read_input_for_day(16);
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("\tPart1: {part1}");
    println!("\tPart2: {part2}");
}

#[derive(Debug)]
struct Tile {
    ch: char,
    energized: bool,
}

impl Tile {
    fn new(ch: char) -> Self {
        Self {
            ch: ch,
            energized: false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

type Coordinate = (isize, isize);
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    coor: Coordinate,
    dir: Direction,
    prev_coor: Option<Coordinate>,
}

impl Beam {
    fn new(coor: Coordinate, dir: Direction) -> Self {
        Self {
            coor: coor,
            dir: dir,
            prev_coor: None,
        }
    }

    fn go(&self, dir: Direction) -> Self {
        let coor = match dir {
            Direction::Right => (self.coor.0, self.coor.1 + 1),
            Direction::Down => (self.coor.0 + 1, self.coor.1),
            Direction::Left => (self.coor.0, self.coor.1 - 1),
            Direction::Up => (self.coor.0 - 1, self.coor.1),
        };
        let mut next = Self::new(coor, dir);
        next.prev_coor = Some(self.coor);
        next
    }
}

fn solve_part1(input: &String) -> String {
    let mut grid = input
        .split('\n')
        .map(|row| row.chars().map(|ch| Tile::new(ch)).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();
    let mut beams = Vec::<Beam>::new();
    let beam = Beam::new((0, 0), Direction::Right);
    beams.push(beam);

    let mut visited = HashSet::<Beam>::new();
    while beams.len() > 0 {
        let beam = beams.remove(0);
        let (r, c) = beam.coor;
        if r < 0 || r >= grid.len() as isize || c < 0 || c >= grid[0].len() as isize {
            continue;
        }
        let (r, c) = (r as usize, c as usize);
        let tile = &mut grid[r][c];
        tile.energized = true;

        match tile.ch {
            '.' => {
                let next = beam.go(beam.dir);
                if !visited.contains(&next) {
                    beams.push(next);
                    visited.insert(next.clone());
                }
            }
            '\\' => {
                let next = beam.go(match beam.dir {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Left,
                });
                if !visited.contains(&next) {
                    beams.push(next);
                    visited.insert(next.clone());
                }
            }
            '|' => {
                match beam.dir {
                    Direction::Right | Direction::Left => {
                        let next = beam.go(Direction::Up);
                        if !visited.contains(&next) {
                            beams.push(next);
                            visited.insert(next.clone());
                        }
                        let next = beam.go(Direction::Down);
                        if !visited.contains(&next) {
                            beams.push(next);
                            visited.insert(next.clone());
                        }
                    }
                    direction => {
                        let next = beam.go(direction);
                        if !visited.contains(&next) {
                            beams.push(next);
                            visited.insert(next.clone());
                        }
                    }
                };
            }
            '-' => {
                match beam.dir {
                    Direction::Down | Direction::Up => {
                        let next = beam.go(Direction::Left);
                        if !visited.contains(&next) {
                            beams.push(next);
                            visited.insert(next.clone());
                        }
                        let next = beam.go(Direction::Right);
                        if !visited.contains(&next) {
                            beams.push(next);
                            visited.insert(next.clone());
                        }
                    }
                    direction => {
                        let next = beam.go(direction);
                        if !visited.contains(&next) {
                            beams.push(next);
                            visited.insert(next.clone());
                        }
                    }
                };
            }
            '/' => {
                let next = beam.go(match beam.dir {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                });
                if !visited.contains(&next) {
                    beams.push(next);
                    visited.insert(next.clone());
                }
            }
            _ => panic!("Unknown tile char {}", tile.ch),
        }
    }

    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|t| t.energized)
                .collect::<Vec<&Tile>>()
                .len()
        })
        .fold(0, |acc, len| acc + len)
        .to_string()
}

fn solve_part2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = [r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."];
    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "46");
    }

    #[test]
    #[ignore]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
