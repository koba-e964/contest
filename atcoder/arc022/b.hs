main = do
  n <- fmap read getLine :: IO Int
  ls <- fmap (map read . words) getLine :: IO [Int]
  print n
  print ls
  print $ check n ls

check :: [Int] -> Int
check ls = 