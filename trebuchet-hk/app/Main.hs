module Main (main) where

import Second

main :: IO ()
main = do
  input <- readFile "input.txt"
  let output = solve input

  print output
