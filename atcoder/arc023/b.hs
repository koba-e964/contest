{-# LANGUAGE BangPatterns #-}

import Control.Monad
import Data.Array.IO

ints = fmap (map read . words) getLine :: IO [Int]

rec r c d ary = do
  sub 0 0 0 where
  sub !i !j !m
    | i == r = return m
    | j == c = sub (i+1) 0 m
    | i+j <= d && (i+j+d) `mod` 2 == 0 = do
      x <- readArray ary (i,j)
      sub i (j+1) (max m x)
    | otherwise = sub i (j+1) m 


main = do
  [r,c,d] <- fmap (map read . words) getLine :: IO [Int]
  ary <- newArray ((0,0), (r-1,c-1)) 0 :: IO (IOArray (Int,Int) Int)
  m <- return 0
  forM_ [0..r-1] $ \i -> do
    ls <- fmap (map read . words) getLine :: IO [Int]
    let {
      sub _ [] = return ();
      sub !k (x:xs) = writeArray ary (i,k) x >> sub (k+1) xs;
    }
    sub 0 ls
  rec r c d ary >>=print

