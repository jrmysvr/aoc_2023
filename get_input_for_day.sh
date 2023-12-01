day="$1"
mkdir -p inputs
curl \
    --cookie "$AOC_SESSION_TOKEN" \
    "https://adventofcode.com/2023/day/$day/input" \
    > "inputs/day$day.txt"
