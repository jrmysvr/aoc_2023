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
    .replace(" ", "")
    .parseInt
  let distances = input
    .split('\n')[1]
    .split(':')[1]
    .replace(" ", "")
    .parseInt
  result = @[(times, distances)]


proc solvePart1(input: string): string =
  let records = parseInputPart1(input)
  result = $records
    .map(rec => toSeq(0..rec.time)
      .filter(t => (t * (rec.time - t)) > rec.distance).len)
    .prod

proc solvePart2(input: string): string =
  let records = parseInputPart2(input)
  result = $records
    .map(rec => toSeq(0..rec.time)
      .filter(t => (t * (rec.time - t)) > rec.distance).len)
    .prod

proc run*() =
  echo "Day 6 Solutions"
  echo "----------------"
  let input = readInputForDay(6)

  echo "Part 1: ", solvePart1(input)
  echo "Part 2: ", solvePart2(input)
