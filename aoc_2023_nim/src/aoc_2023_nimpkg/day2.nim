import input
import std/[enumerate, math, sequtils, strutils, tables]

const bag = {"red": 12, "green": 13, "blue": 14}.toTable

proc solvePart1(input: string): string =
  var idSum = 0
  for i, gameLine in enumerate(input.split('\n')):
    let id = i + 1
    let game = gameLine.strip.split(':')[1]
    var possible = true
    for grab in game.split(';'):
      for blocks in grab.strip.split(','):
        let countColor = blocks.strip.split(' ')
        let (count, color) = (countColor[0], countColor[1])
        if parseInt(count) > bag[color]:
          possible = false
          break
    if possible:
      idSum += id

  result = $idSum


proc solvePart2(input: string): string =
  var powerSum = 0
  for gameLine in input.split('\n'):
    let game = gameLine.strip.split(':')[1]
    var colorCounts = {"red": 0, "green": 0, "blue": 0}.toTable
    for grab in game.split(';'):
      for blocks in grab.strip.split(','):
        let countColor = blocks.strip.split(' ')
        let (count, color) = (countColor[0], countColor[1])
        colorCounts[color] = max(colorCounts[color], parseInt(count))

    powerSum += prod(colorCounts.values.toSeq)

  result = $powerSum



proc run*() =
  echo "Day 2 Solutions"
  echo "------------------"
  let input = readInputForDay(2)
  let part1 = solvePart1(input)
  let part2 = solvePart2(input)

  echo "Part 1: ", solvePart1(input)
  echo "Part 2: ", solvePart2(input)
