module Main (main) where

import Second

main :: IO ()
main = do
  input <- readFile "input.txt"
  print (solve input)
