module StringHelpers
  ( trimStart,
    indexOf,
    splitOff,
    splitBy,
  )
where

import Data.Bifunctor (Bifunctor (second))

trimStart :: String -> String
trimStart = dropWhile (== ' ')

indexOf :: Char -> String -> Int
indexOf char = length . takeWhile (/= char)

splitOff :: Char -> String -> (String, String)
splitOff delimiter input = second (drop 1) (splitAt (indexOf delimiter input) input)

splitBy :: Char -> String -> [String]
splitBy _ "" = []
splitBy delimiter input = (\(first_part, rest) -> first_part : splitBy delimiter rest) (splitOff delimiter input)
