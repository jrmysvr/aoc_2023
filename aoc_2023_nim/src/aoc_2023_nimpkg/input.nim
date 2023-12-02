import strformat
import strutils
import os

proc readInputForDay*(day_num: int) : string =
  let dirName = getEnv("AOC_INPUT_DIR")
  let inputDir = absolutePath(dirName)
  let fileName = fmt"day{day_num}.txt"
  let filePath = inputDir / fileName
  result = readFile(filePath).strip
