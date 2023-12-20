#![allow(dead_code)]
#![allow(unused_variables)]

use crate::aoc::input::read_input_for_day;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

pub fn run() {
    println!("Day 17 Solutions");
    println!("---------------");
    let input = read_input_for_day(17);
    println!("Skipping day 17 for now...");
    // let part1 = solve_part1(&input);
    // let part2 = solve_part2(&input);
    // println!("\tPart1: {part1}");
    // println!("\tPart2: {part2}");
}

// Adapted from https://rosettacode.org/wiki/Dijkstra%27s_algorithm#Rust
struct Grid<T> {
    nodes: Vec<Node<T>>,
}

struct Node<T> {
    data: T,
    edges: Vec<(usize, Block)>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node: usize,
    cost: Block,
}

// Manually implement Ord so we get a min-heap instead of a max-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type WeightedEdge = (usize, usize, Block);

impl<T> Grid<T> {
    fn new() -> Self {
        Grid { nodes: Vec::new() }
    }

    fn add_node(&mut self, data: T) -> usize {
        let node = Node {
            edges: Vec::new(),
            data: data,
        };
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn create_edges<'a, I>(&mut self, iterator: I)
    where
        I: IntoIterator<Item = &'a WeightedEdge>,
    {
        for &(start, end, weight) in iterator.into_iter() {
            self.nodes[start].edges.push((end, weight));
            self.nodes[end].edges.push((start, weight));
        }
    }

    fn find_path(&self, start: usize, end: usize) -> Option<(Vec<usize>, Block)> {
        let mut dist = vec![(Block::MAX, None); self.nodes.len()];

        let mut heap = BinaryHeap::new();
        dist[start] = (0, None);
        heap.push(State {
            node: start,
            cost: 0,
        });

        while let Some(State { node, cost }) = heap.pop() {
            if node == end {
                let mut path = Vec::with_capacity(dist.len() / 2);
                let mut current_dist = dist[end];
                path.push(end);
                while let Some(prev) = current_dist.1 {
                    path.push(prev);
                    current_dist = dist[prev];
                }
                path.reverse();
                return Some((path, cost));
            }

            if cost > dist[node].0 {
                continue;
            }
            for edge in &self.nodes[node].edges {
                let next = State {
                    node: edge.0,
                    cost: cost + edge.1,
                };
                if next.cost < dist[next.node].0 {
                    dist[next.node] = (next.cost, Some(node));
                    heap.push(next);
                }
            }
        }
        None
    }
}

type Block = i32;
type Blocks = Vec<Block>;
type City = Vec<Blocks>;
type Coordinate = (usize, usize);

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse_city_from(input: &String) -> City {
    input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|block| block.to_digit(10).unwrap() as Block)
                .collect::<Blocks>()
        })
        .collect::<City>()
}

fn calc_index_from_row_col(row: usize, col: usize, city: &City) -> usize {
    let n_cols = city[0].len();
    (n_cols * row + col) as usize
}

fn solve_part1(input: &String) -> String {
    let city = parse_city_from(input);
    let mut grid = Grid::new();
    let mut nodes = Vec::<usize>::new();
    let mut edges: Vec<(usize, usize, Block)> = Vec::new();
    let dx: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for (r, row) in city.iter().enumerate() {
        for (c, block) in row.iter().enumerate() {
            let node = grid.add_node(((r, c), *block));
            nodes.push(node);
            for (i, (rx, cx)) in dx.into_iter().enumerate() {
                let (r, c) = (r as isize, c as isize);
                if r + rx < 0
                    || (r + rx) >= city.len() as isize
                    || c + cx < 0
                    || (c + cx) >= city[0].len() as isize
                {
                    continue;
                }
                let ix = calc_index_from_row_col((r + rx) as usize, (c + cx) as usize, &city);
                edges.push((node, ix, *block));
            }
        }
    }
    grid.create_edges(&edges[..]);
    let start = 0;
    let end = nodes.len() - 1;
    println!("{:?}, {:?}", nodes[start], nodes[end - 1]);
    let (path, cost) = grid.find_path(start, end).unwrap();
    print!("{:?}", grid.nodes[start].data);
    for i in path.iter().skip(1) {
        print!(" -> {:?}", grid.nodes[*i].data);
    }
    cost.to_string()
}

fn solve_part2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [&str; 1] = ["
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_calc_index() {
        let city = parse_city_from(&get_input(0));
        let row_col_ix = vec![(0, 0, 0), (0, 1, 1), (1, 0, 13), (1, 1, 14), (2, 1, 27)];
        for (r, c, expected_ix) in row_col_ix {
            let actual_ix = calc_index_from_row_col(r, c, &city);
            assert_eq!(actual_ix, expected_ix);
        }
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "102");
    }

    #[test]
    #[ignore]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "");
    }
}
