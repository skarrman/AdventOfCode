{-# LANGUAGE ImportQualifiedPost #-}

import Control.Monad.RWS (MonadState (put))
import Data.Map (Map, fromList, keys, (!))
import Data.Set qualified as Set
import System.Environment (lookupEnv)

splitPath :: [Char] -> [Char] -> ([Char], Map String (String, String))
splitPath ('\n' : '\n' : m) p = (p, makeMap $ lines m)
splitPath (p' : m) p = splitPath m (p ++ [p'])

extract :: [Char] -> (String, (String, String))
extract [k, e, y, _, _, _, _, l, e', f, _, _, r, i, g, _] = ([k, e, y], ([l, e', f], [r, i, g]))

makeMap :: [String] -> Map String (String, String)
makeMap ls = fromList $ map extract ls

walk :: Map String (String, String) -> [Char] -> [Char] -> String -> Int -> Int
walk _ _ _ "ZZZ" s = s
walk m [] p k s = walk m p p k s
walk m ('L' : p) p' k s = walk m p p' (fst (m ! k)) s + 1
walk m ('R' : p) p' k s = walk m p p' (snd (m ! k)) s + 1

ghostWalk :: Map String (String, String) -> [Char] -> [Char] -> String -> Int -> Int
ghostWalk _ _ _ [_, _, 'Z'] s = s
ghostWalk m [] p k s = ghostWalk m p p k s
ghostWalk m ('L' : p) p' k s = ghostWalk m p p' (fst (m ! k)) s + 1
ghostWalk m ('R' : p) p' k s = ghostWalk m p p' (snd (m ! k)) s + 1

ghosts :: Map String (String, String) -> [String]
ghosts m = filter ghostStart $ keys m

ghostStart :: String -> Bool
ghostStart [_, _, 'A'] = True
ghostStart _ = False

walkGhosts :: [String] -> Map String (String, String) -> [Char] -> [Int] -> Int
walkGhosts [] _ _ ls = foldl (flip lcm) 1 ls
walkGhosts (s : ss) m p ls = walkGhosts ss m p (ls ++ [ghostWalk m p p s 0])

solve :: String -> ([Char], Map String (String, String)) -> String
solve "part1" (p, m) = show $ walk m p p "AAA" 0
solve "part2" (p, m) = show $ walkGhosts (ghosts m) m p []

whichPart part = case part of
  Just (p) -> p
  Nothing -> "part1"

main :: IO ()
main = do
  content <- readFile "input.txt"
  part <- whichPart <$> lookupEnv "part"
  putStrLn $ solve part $ splitPath content []