# TODO: Replace this with a nim version in the nimble file?
#
day="$1"
if [ -z "$day" ]
then
    echo "Usage: $0 <day number>"
    exit 1
fi

fpath="src/aoc_2023_nimpkg/day$day.nim"

if [ -f "$fpath" ]
then
    echo "File \"$fpath\" already exists!"
    exit 1
fi

echo "Creating a new solution file: $fpath"

cat > "$fpath" <<EOF
import input


proc solvePart1(input: string): string =
  result = ""


proc solvePart2(input: string): string =
  result = ""


proc run*() =
  echo "Day 2 Solutions"
  echo "------------------"
  let input = readInputForDay(2)
  let part1 = solvePart1(input)
  let part2 = solvePart2(input)

  echo "Part 1: ", solvePart1(input)
  echo "Part 2: ", solvePart2(input)
EOF
