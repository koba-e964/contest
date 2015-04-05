import Data.Char
main = do
  line <- getLine
  putStrLn (if check (map toLower line) "ict" then "YES" else "NO")

check :: String -> String -> Bool
check _ [] = True
check [] _ = False
check (x:xs) (y:ys)
  | x == y = check xs ys
  | otherwise = check xs (y:ys)
  