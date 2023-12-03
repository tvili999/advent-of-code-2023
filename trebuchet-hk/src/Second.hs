module Second
  ( solve,
  )
where

import Data.Foldable
import Data.List (isPrefixOf, isSuffixOf)

digitLookupList :: [(String, Integer)]
digitLookupList =
  [ ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9)
  ]

toDigit :: String -> Integer
toDigit input = head (map snd (filter (\(digit, _) -> input == digit) digitLookupList))

firstSubstring :: String -> [String] -> Maybe String
firstSubstring "" _ = Nothing
firstSubstring line substrings = case find (`isPrefixOf` line) substrings of
  Just res -> Just res
  Nothing -> firstSubstring (tail line) substrings

lastSubstring :: String -> [String] -> Maybe String
lastSubstring "" _ = Nothing
lastSubstring line substrings = case find (`isSuffixOf` line) substrings of
  Just res -> Just res
  Nothing -> lastSubstring (init line) substrings

value :: String -> Integer
value line = case ( firstSubstring line (map fst digitLookupList),
                    lastSubstring line (map fst digitLookupList)
                  ) of
  (Just firstDigit, Just lastDigit) -> toDigit (firstDigit) * 10 + toDigit (lastDigit)
  _ -> 0

solve :: String -> Integer
solve input = sum (map value (lines input))
