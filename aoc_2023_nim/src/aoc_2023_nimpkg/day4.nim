import input
import std/[math, sequtils, sets, strutils]

proc convertInputToCardCounts(input: string): seq[int] =
  var counts: seq[int] = @[]
  for card in input.splitLines:
    let line = card.split(':')[1]
    let row = line.split('|')
    let (winning, numbers) = (row[0].strip, row[1].strip)
    let winningNumbers = toHashSet(
      winning.split(' ').filterIt(it != "").mapIt(parseInt(it))
    )
    counts.add(numbers
      .split(' ')
      .filterIt(it != "")
      .mapIt(parseInt(it))
      .filterIt(it in winningNumbers)
      .len
    )

  counts

proc solvePart1(input: string): string =
  let counts = convertInputToCardCounts(input)
  result = $counts.mapIt(if it > 0: 2^(it-1) else: 0).sum

proc solvePart2(input: string): string =
  let counts = convertInputToCardCounts(input)
  var wonCards: seq[int] = newSeqWith(counts.len, 0)
  for id in 0..counts.len-1:
    wonCards[id] += 1

    for ix in 1..counts[id]:
      wonCards[id + ix] += wonCards[id]


  result = $wonCards.sum


proc run*() =
  echo "Day 4 Solutions"
  echo "----------------"
  let input = readInputForDay(4)

  echo "Part 1: ", solvePart1(input)
  echo "Part 2: ", solvePart2(input)
