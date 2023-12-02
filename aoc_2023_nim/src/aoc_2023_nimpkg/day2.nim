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
