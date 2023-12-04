module Second
  ( solve,
  )
where

import Data.Maybe (fromMaybe)
import StringHelpers

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

powerOfCubes :: (Integer, Integer, Integer) -> Integer
powerOfCubes (r, g, b) = r * g * b

solveLine :: String -> Integer
solveLine line = powerOfCubes (largestGrabs (map parseGrab (parseGame line)))

solve :: String -> Integer
solve input = sum (map solveLine (lines input))
