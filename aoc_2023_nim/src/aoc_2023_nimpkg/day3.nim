import input
import std/[enumerate, math, strutils, sugar]

type
  Cell = object
    val: string
    row: int
    colRng: (int, int)

  Coordinate = tuple
    row: int
    col: int

proc newCell(val: string, coor: Coordinate): Cell =
  Cell(
    val: val,
    row: coor.row,
    colRng: (coor.col, coor.col + val.len)
  )

proc isNumber(cell: Cell): bool =
  result = cell.val[0].isDigit

proc isSymbol(cell: Cell): bool =
  result = not cell.isNumber

proc isGear(cell: Cell): bool =
  result = cell.val == "*"

proc numVal(cell: Cell): int =
  result = if cell.isNumber: cell.val.parseInt else: -1

proc isNeighborOfCoor(cell: Cell, coor: Coordinate): bool =
  for rx in [-1, 0, 1]:
    for cx in [-1, 0, 1]:
      if rx == 0 and cx == 0: continue
      var
        row = max(coor.row + rx, 0)
        col = max(coor.col + cx, 0)
      if cell.row == row and
      cell.colRng[0] <= col and
      cell.colRng[1] > col: return true

  false

proc isNeighborOf(cell: Cell, other: Cell): bool =
  for c in (other.colRng[0] .. other.colRng[1]-1):
    if cell.isNeighborOfCoor((other.row, c)):
      return true

  false

proc convertInputToCells(input: string): seq[Cell] =
  var cells: seq[Cell] = @[]
  for (row, line) in enumerate(input.splitLines):
    var numStr = ""
    for (col, ch) in enumerate(line):
      if ch.isDigit:
        numStr &= ch
      else:
        if numStr.len > 0:
          cells.add(newCell(numStr, (row, col - numStr.len)))
          numStr = ""
        if ch != '.':
          cells.add(newCell("" & ch, (row, col)))

    if numStr.len > 0:
      cells.add(newCell(numStr, (row, line.len - numStr.len)))

  cells

proc solvePart1(input: string): string =
  let cells = convertInputToCells(input)
  var sum = 0
  let symbolCells = collect(newSeq):
    for cell in cells:
      if cell.isSymbol: cell

  for symbolCell in symbolCells:
    for cell in cells:
      if cell.isNumber and cell.isNeighborOf(symbolCell):
        sum += cell.numVal

  result = $sum


proc solvePart2(input: string): string =
  let cells = convertInputToCells(input)
  var sum = 0
  let gearCells = collect(newSeq):
    for cell in cells:
      if cell.isGear: cell

  for gearCell in gearCells:
    var nums: seq[int] = @[]
    for cell in cells:
      if cell.isNumber and cell.isNeighborOf(gearCell):
        nums.add(cell.numVal)
    if nums.len == 2:
      sum += prod(nums)

  result = $sum

proc run*() =
  echo "Day 3 Solutions"
  echo "----------------"
  let input = readInputForDay(3)
  echo "Part 1: ", solvePart1(input.strip)
  echo "Part 2: ", solvePart2(input.strip)
