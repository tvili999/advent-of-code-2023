module First
  ( solve,
  )
where

import Data.Maybe (fromMaybe)
import StringHelpers

parseGameId :: String -> Integer
parseGameId input = read (init (words input !! 1))

parseGame :: String -> [String]
parseGame game = splitBy ';' (tail (dropWhile (/= ':') game))

parseGrabColorList :: String -> [(String, Integer)]
parseGrabColorList grab = map ((\[x, y] -> (x, read y)) . reverse . words . trimStart) (splitBy ',' grab)

parseGrab :: String -> (Integer, Integer, Integer)
parseGrab grab =
  let table = parseGrabColorList grab
   in (fromMaybe 0 (lookup "red" table), fromMaybe 0 (lookup "green" table), fromMaybe 0 (lookup "blue" table))

largestGrabs :: [(Integer, Integer, Integer)] -> (Integer, Integer, Integer)
largestGrabs grabs =
  ( maximum (map (\(x, _, _) -> x) grabs),
    maximum (map (\(_, x, _) -> x) grabs),
    maximum (map (\(_, _, x) -> x) grabs)
  )

possible :: (Integer, Integer, Integer) -> Bool
possible (r, g, b) = r <= 12 && g <= 13 && b <= 14

solveLine :: String -> Bool
solveLine line = possible (largestGrabs (map parseGrab (parseGame line)))

possibleGames :: [String] -> [(Integer, Bool)]
possibleGames game = filter snd (map (\line -> (parseGameId line, solveLine line)) game)

solve :: String -> Integer
solve input = sum (map fst (possibleGames (lines input)))
