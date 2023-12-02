# Package

version       = "0.1.0"
author        = "Jeremy Savor"
description   = "My solutions for Advent of Code 2023"
license       = "MIT"
srcDir        = "src"
installExt    = @["nim"]
bin           = @["aoc_2023_nim"]


# Dependencies

requires "nim >= 2.0.0"

task aoc, "Run all solutions":
  putEnv("AOC_INPUT_DIR", "../inputs")
  exec "nimble run"

after aoc:
  exec "rm aoc_2023_nim"
