## Solutions for Advent of Code 2023

https://adventofcode.com

### Rust Solutions
```shell
cd aoc_2022_rust
cargo test
cargo run
```

## Fetch input for a given day

**_Do not overload the AOC server!_**

```shell
curl --cookie "$AOC_SESSION_TOKEN" https://adventofcode.com/2022/day/1/input
```
where `AOC_SESSION_TOKEN` has the value of the `session` token in a cookie found using
a browser and navigating to a daily input page for the first time.

```
export AOC_SESSION_TOKEN="session=..."
```
