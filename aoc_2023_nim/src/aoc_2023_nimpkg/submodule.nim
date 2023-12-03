import day1
import day2
import day3

var allDays = @[
  day1.run,
  day2.run,
  day3.run,
]

proc runAll*() =
  for day in allDays:
    day()
    echo ""
