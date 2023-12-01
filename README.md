## Solutions for Advent of Code 2023

https://adventofcode.com

### Rust Solutions
```shell
cd aoc_2023_rust
cargo test
cargo run
```

## Fetch input for a given day

**_Do not overload the AOC server!_**

Set an environment variable to be `AOC_SESSION_TOKEN` with the value of the `session` token in a cookie found using
a browser and navigating to a daily input page for the first time.

```
export AOC_SESSION_TOKEN="session=..."
```

and run

```shell
bash get_input_for_day.sh <day number>
```

This script will save the input in a directory called `inputs/` in the root directory of this repo.
