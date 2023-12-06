#![allow(dead_code)]
#![allow(unused_variables)]

use crate::aoc::input::read_input_for_day;
use std::collections::HashMap;

pub fn run() {
    println!("Day 5 Solutions");
    println!("---------------");
    let input = read_input_for_day(5);
    let part1 = solve_part1(&input);
    println!("\tPart1: {part1}");
    let part2 = solve_part2(&input);
    println!("\tPart2: {part2}");
}

type Num = i64;
type Seeds = Vec<Num>;

fn parse_seeds_from(input: &String) -> Seeds {
    let seed_line = input.split('\n').nth(0).unwrap();
    assert!(seed_line.starts_with("seeds"));
    seed_line
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| Num::from_str_radix(s, 10).unwrap())
        .collect::<Seeds>()
}

#[derive(Debug)]
struct Mapping {
    src: Num,
    dst: Num,
    rng: Num,
}

#[derive(Debug)]
struct Map {
    src_name: String,
    dst_name: String,
    mappings: Vec<Mapping>,
}

impl Mapping {
    fn new(src: Num, dst: Num, rng: Num) -> Self {
        Self {
            src: src,
            dst: dst,
            rng: rng,
        }
    }
}

impl Map {
    fn new(name: String, mappings: Vec<Mapping>) -> Self {
        let src_name = name.split("-to-").nth(0).unwrap();
        let dst_name = name.split("-to-").nth(1).unwrap();

        Self {
            src_name: src_name.to_string(),
            dst_name: dst_name.to_string(),
            mappings: mappings,
        }
    }
}

fn parse_map_from(input: &String) -> Map {
    let name = input.split('\n').nth(0).unwrap().split(' ').nth(0).unwrap();
    let mut mappings = Vec::<Mapping>::new();
    for line in input.split('\n').skip(1) {
        let dst = Num::from_str_radix(line.split(' ').nth(0).unwrap(), 10).unwrap();
        let src = Num::from_str_radix(line.split(' ').nth(1).unwrap(), 10).unwrap();
        let rng = Num::from_str_radix(line.split(' ').nth(2).unwrap(), 10).unwrap();
        mappings.push(Mapping::new(src, dst, rng));
    }
    Map::new(String::from(name), mappings)
}

fn parse_maps_from(input: &String, src_map: bool) -> HashMap<String, Map> {
    let mut maps = HashMap::<String, Map>::new();
    for map_input in input.split("\n\n").skip(1) {
        let map = parse_map_from(&map_input.to_string());
        let name = if src_map { map.src_name.to_owned() } else { map.dst_name.to_owned() };
        maps.insert(name, map);
    }

    maps
}

fn calc_dst_for_src(src_name: &str, src_value: Num, maps: &HashMap<String, Map>) -> (String, Num) {
    let map = &maps[src_name];
    let mut value = src_value;
    for mapping in map.mappings.iter() {
        let &Mapping{ src, dst, rng } = mapping;
        if src <= src_value && src_value < (src + rng) {
            value = dst + (src_value - src);
        }

    }
    (map.dst_name.clone(), value)
}

fn calc_src_for_dst(dst_name: &str, dst_value: Num, maps: &HashMap<String, Map>) -> (String, Num) {
    let map = &maps[dst_name];
    let mut value = dst_value;
    for mapping in map.mappings.iter() {
        let &Mapping{ src, dst, rng } = mapping;
        if dst <= dst_value && dst_value < (dst + rng) {
            value = src + (dst_value - dst);
        }

    }
    (map.src_name.clone(), value)
}

// TODO: Basically, check if the seed value could come from any of the locations.
//       - If so, then perform some sort of search (binary?) to find the smallest location?


fn calc_location_for_seed(seed: Num, maps: &HashMap<String, Map>) -> Num {
    let mut src_name = String::from("seed");
    let mut src_value = seed;
    let location = "location".to_string();
    while src_name != location {
        let (dst_name, dst_value) = calc_dst_for_src(&src_name[..], src_value, maps);
        src_name = dst_name.clone();
        src_value = dst_value;
    }

    src_value
}

// TODO: consolidate this with `calc_location_for_seed
fn calc_seed_from_location(location: Num, maps: &HashMap<String, Map>) -> Num {
    let mut dst_name = String::from("location");
    let mut dst_value = location;
    let seed = "seed".to_string();
    while dst_name != seed {
        let (src_name, src_value) = calc_src_for_dst(&dst_name[..], dst_value, maps);
        dst_name = src_name.clone();
        dst_value = src_value;
    }

    dst_value   
}

fn is_valid_seed(seed: Num, all_seeds: &Vec<Num>) -> bool {
    for i in (0..all_seeds.len()-1).step_by(2) {
        let (current_seed, seed_rng) = (all_seeds[i], all_seeds[i+1]);
        if current_seed <= seed && seed <= current_seed + seed_rng { return true; }
    }

    false
}

// Determine the "closest" location that needs a seed
//  - Which basically means find the smallest location value that's found through
//    a series of mappings in the given input string.
fn solve_part1(input: &String) -> String {
    let input = String::from(input.trim()); // :(
    let seeds = parse_seeds_from(&input);
    let maps = parse_maps_from(&input, true);
    let mut lowest_location = Num::MAX;
    for seed in seeds {
        let current_location = calc_location_for_seed(seed, &maps);
        lowest_location = std::cmp::min(lowest_location, current_location);
    }

    lowest_location.to_string()
}

// Determine the "closest" location that needs a seed based on a range of seed values
fn solve_part2(input: &String) -> String {
    let input = String::from(input.trim()); // :(
    let seeds = parse_seeds_from(&input);
    let starting_seeds = seeds.iter().step_by(2);
    let maps = parse_maps_from(&input, true);
    let mut lowest_location = Num::MAX;
    for seed in starting_seeds {
        let current_location = calc_location_for_seed(*seed, &maps);
        lowest_location = std::cmp::min(lowest_location, current_location);
    }

    let rev_maps = parse_maps_from(&input, false);
    let mut location = lowest_location;
    loop {
        location -= 1;
        let seed = calc_seed_from_location(location, &rev_maps);
        if is_valid_seed(seed, &seeds) {
            lowest_location = std::cmp::min(lowest_location, location);
        }

        if location == 0 { break; }
    }

    lowest_location.to_string()
}

#[cfg(test)]
mod test {

    use super::*;

    const INPUT: [&str; 1] = ["
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"];

    fn get_input(ix: usize) -> String {
        String::from(INPUT[ix].trim())
    }

    #[test]
    fn test_parse_seeds_from() {
        let seeds = parse_seeds_from(&get_input(0));
        let expected_seeds = Seeds::from([79, 14, 55, 13]);
        assert_eq!(seeds.len(), expected_seeds.len());
        for (actual, expected) in seeds.iter().zip(expected_seeds.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_parse_map_from() {
        let full_input = get_input(0);
        // Get seed-to-soil map
        let input = String::from(full_input.split("\n\n").nth(1).unwrap());
        let map = parse_map_from(&input);
        let Mapping{src, dst, rng} = map.mappings[0];
        assert_eq!(src, 98);
        assert_eq!(dst, 50);
        assert_eq!(rng, 2);
    }

    #[test]
    fn test_calc_dst_for_src() {
        let seeds = Seeds::from([79, 14, 55, 13]);
        let expected_values = [81, 14, 57, 13];
        let expected_dst_name = "soil";
        let src_name = "seed";
        let maps = parse_maps_from(&get_input(0), true);
        for (value, expected_value) in seeds.iter().zip(expected_values.iter()) {
            let (dst_name, dst_value) = calc_dst_for_src(src_name, *value, &maps);
            assert_eq!(dst_name, expected_dst_name);
            assert_eq!(dst_value, *expected_value);
        }

        let soils = Vec::<Num>::from([81, 14, 57, 13]);
        let expected_values = [81, 53, 57, 52];
        let expected_dst_name = "fertilizer";
        let src_name = "soil";
        let maps = parse_maps_from(&get_input(0), true);
        for (value, expected_value) in soils.iter().zip(expected_values.iter()) {
            let (dst_name, value) = calc_dst_for_src(src_name, *value, &maps);
            assert_eq!(dst_name, expected_dst_name);
            assert_eq!(value, *expected_value);
        }

        let fertilizers = Vec::<Num>::from([81, 53, 57, 52]);
        let expected_values = [81, 49, 53, 41];
        let expected_dst_name = "water";
        let src_name = "fertilizer";
        let maps = parse_maps_from(&get_input(0), true);
        for (value, expected_value) in fertilizers.iter().zip(expected_values.iter()) {
            let (dst_name, value) = calc_dst_for_src(src_name, *value, &maps);
            assert_eq!(dst_name, expected_dst_name);
            assert_eq!(value, *expected_value);
        }

        let water = Vec::<Num>::from([81, 49, 53, 41]);
        let expected_values = [74, 42, 46, 34];
        let expected_dst_name = "light";
        let src_name = "water";
        let maps = parse_maps_from(&get_input(0), true);
        for (value, expected_value) in water.iter().zip(expected_values.iter()) {
            let (dst_name, value) = calc_dst_for_src(src_name, *value, &maps);
            assert_eq!(dst_name, expected_dst_name);
            assert_eq!(value, *expected_value);
        }

        let light = Vec::<Num>::from([74, 42, 46, 34]);
        let expected_values = [78, 42, 82, 34];
        let expected_dst_name = "temperature";
        let src_name = "light";
        let maps = parse_maps_from(&get_input(0), true);
        for (value, expected_value) in light.iter().zip(expected_values.iter()) {
            let (dst_name, value) = calc_dst_for_src(src_name, *value, &maps);
            assert_eq!(dst_name, expected_dst_name);
            assert_eq!(value, *expected_value);
        }

        let temperatures = Vec::<Num>::from([78, 42, 82, 34]);
        let expected_values = [78, 43, 82, 35]; 
        let expected_dst_name = "humidity";
        let src_name = "temperature";
        let maps = parse_maps_from(&get_input(0), true);
        for (value, expected_value) in temperatures.iter().zip(expected_values.iter()) {
            let (dst_name, value) = calc_dst_for_src(src_name, *value, &maps);
            assert_eq!(dst_name, expected_dst_name);
            assert_eq!(value, *expected_value);
        }

        let humidity = Vec::<Num>::from([78, 43, 82, 35]);
        let expected_values = [82, 43, 86, 35];
        let expected_dst_name = "location";
        let src_name = "humidity";
        let maps = parse_maps_from(&get_input(0), true);
        for (value, expected_value) in humidity.iter().zip(expected_values.iter()) {
            let (dst_name, value) = calc_dst_for_src(src_name, *value, &maps);
            assert_eq!(dst_name, expected_dst_name);
            assert_eq!(value, *expected_value);
        }
    }

    #[test]
    fn test_calc_location_for_seed() {
        let seeds = Seeds::from([79, 14, 55, 13]);
        let expected_locations = [82, 43, 86, 35];
        let maps = parse_maps_from(&get_input(0), true);
        for (seed, expected_location) in seeds.iter().zip(expected_locations.iter()) {
            let location = calc_location_for_seed(*seed, &maps);
            assert_eq!(location, *expected_location);
        }
    }

    #[test]
    fn test_calc_seed_from_location() {
        let locations = vec![82, 43, 86, 35];
        let expected_seeds = [79, 14, 55, 13];
        let maps = parse_maps_from(&get_input(0), false);
        assert_eq!(locations.len(), expected_seeds.len());
        for (location, expected_seed) in locations.iter().zip(expected_seeds.iter()) {
            let seed = calc_seed_from_location(*location, &maps);
            assert_eq!(seed, *expected_seed);
        }
    }

    #[test]
    fn test_reverse_mapping() {
        let maps = parse_maps_from(&get_input(0), false);
        let dst_name = "location";
        let map = &maps[dst_name];
        assert_eq!(map.src_name, "humidity");
        assert_eq!(map.dst_name, "location");

        let (src_name, value) = calc_src_for_dst(dst_name, 56, &maps);
        assert_eq!(src_name, "humidity");
        assert_eq!(value, 93);
    }

    #[test]
    fn test_is_valid_seed() {
        let seeds = vec![79, 14, 55, 13];
        assert!(is_valid_seed(79, &seeds));
        assert!(is_valid_seed(80, &seeds));
        assert!(is_valid_seed(93, &seeds));
        assert!(is_valid_seed(55, &seeds));
        assert!(is_valid_seed(56, &seeds));
        assert!(is_valid_seed(68, &seeds));
        assert!(!is_valid_seed(78, &seeds));
        assert!(!is_valid_seed(94, &seeds));
        assert!(!is_valid_seed(54, &seeds));
        assert!(!is_valid_seed(70, &seeds));
    }

    #[test]
    fn test_full_part1() {
        assert_eq!(solve_part1(&get_input(0)), "35");
    }

    #[test]
    fn test_full_part2() {
        assert_eq!(solve_part2(&get_input(0)), "46");
    }
}
