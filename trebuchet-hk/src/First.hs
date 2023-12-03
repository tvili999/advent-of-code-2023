module First
  ( solve,
  )
where

import Data.Char

digitsOfLine :: String -> String
digitsOfLine line = [head (filter isDigit line), last (filter isDigit line)]

solve :: String -> Integer
solve input = sum (map (read . digitsOfLine) (lines input))
