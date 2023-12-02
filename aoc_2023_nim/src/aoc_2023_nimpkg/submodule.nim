# This is just an example to get you started. Users of your hybrid library will
# import this file by writing ``import aoc_2023_nimpkg/submodule``. Feel free to rename or
# remove this file altogether. You may create additional modules alongside
# this file as required.

import day1

var allDays = @[
  day1.run
];

proc runAll*() =
  for day in allDays:
    day()
