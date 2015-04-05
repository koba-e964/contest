solve :: Int -> Int -> (Int, Int, Int)
solve n m
  |m < 2* n || m > 4 * n = (-1, -1, -1)
  |m <= 3 * n = let x = m - 2 * n in (n - x, x, 0)
  |otherwise = let x = 4 * n - m in (0, x, n - x)

main = do
  [n, m]<- fmap (map read . words) getContents
  let (a, b, c) = solve n m in 
    putStrLn $ show a ++ " " ++ show b ++ " " ++ show c

