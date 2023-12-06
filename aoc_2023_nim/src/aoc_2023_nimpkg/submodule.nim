import day1
import day2
import day3
import day4
import day6

var allDays = @[
  day1.run,
  day2.run,
  day3.run,
  day4.run,
  day6.run,
]

proc runAll*() =
  for day in allDays:
    day()
    echo ""
