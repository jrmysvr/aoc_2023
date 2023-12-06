import input
import std/[math, sequtils, strutils, sugar]

type
  Record = tuple
    time: int
    distance: int

proc parseInputPart1(input: string): seq[Record] =
  let times: seq[int] = input
    .split('\n')[0]
    .split(':')[1]
    .split(' ')
    .filterIt(len(it) > 0)
    .mapIt(parseInt(it.strip))
  let distances: seq[int] = input
    .split('\n')[1]
    .split(':')[1]
    .split(' ')
    .filterIt(len(it) > 0)
    .mapIt(parseInt(it.strip))
  result = times.zip(distances)

proc parseInputPart2(input: string): seq[Record] =
  let times = input
    .split('\n')[0]
    .split(':')[1]
    .filterIt(it.isDigit)
    .join("")
    .parseInt

  let distances = input
    .split('\n')[1]
    .split(':')[1]
    .filterIt(it.isDigit)
    .join("")
    .parseInt
  result = @[(times, distances)]


proc solvePart1(input: string): string =
  let records = parseInputPart1(input)
  var product = 1
  for record in records:
    var count = 0
    for t in 0..record.time:
      if t * (record.time - t) > record.distance:
        count += 1

    product *= count

  result = $product


proc solvePart2(input: string): string =
  let records = parseInputPart2(input)
  var product = 1
  for record in records:
    var count = 0
    for t in 0..record.time:
      if t * (record.time - t) > record.distance:
        count += 1

    product *= count

  result = $product


proc run*() =
  echo "Day 6 Solutions"
  echo "----------------"
  let input = readInputForDay(6)

  echo "Part 1: ", solvePart1(input)
  echo "Part 2: ", solvePart2(input)
