import input
import strutils

const digStrs = @["1", "2", "3", "4", "5", "6", "7", "8", "9"]
const numStrs = @["one", "two", "three", "four", "five", "six", "seven",
    "eight", "nine"]

func numStrToNum(numStr: string): int =
  result = case numStr:
    of "1", "one": 1
    of "2", "two": 2
    of "3", "three": 3
    of "4", "four": 4
    of "5", "five": 5
    of "6", "six": 6
    of "7", "seven": 7
    of "8", "eight": 8
    of "9", "nine": 9
    else: 0

func getCalibValueFrom(line: string, digitsOnly: bool = true): int =
  var allNums: seq[int] = @[]
  let strs = if digitsOnly: digStrs else: digStrs & numStrs
  for i in 0 .. line.len:
    for str in strs:
      if line[i..line.high].startsWith(str):
        allNums.add(numStrToNum(str))
        continue

  let tens = allNums[0]
  let ones = allNums[^1]

  10 * tens + ones

proc run*() =
  echo "Day 1 Solutions"
  echo "---------------"
  let input = readInputForDay(1)
  var calibSum1 = 0
  var calibSum2 = 0
  for line in input.splitLines:
    let calibValue1 = getCalibValueFrom(line)
    let calibValue2 = getCalibValueFrom(line, digitsOnly=false)
    calibSum1 += calibValue1
    calibSum2 += calibValue2

  echo "Part 1: ", calibSum1
  echo "Part 2: ", calibSum2
