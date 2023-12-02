import day1
import day2

var allDays = @[
  day1.run,
  day2.run,
]

proc runAll*() =
  for day in allDays:
    day()
    echo ""
