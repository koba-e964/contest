{-# LANGUAGE BangPatterns #-}
import Control.Monad
import Control.Monad.ST hiding (unsafeSTToIO, unsafeIOToST)
import Control.Monad.ST.Unsafe
import Data.Array.ST
import Data.Array (Array)
import Debug.Trace

inf = 0x3ffffff ::Int


-- returns min {x <- ary.indices | ary[x] >= val}
search :: (MArray a e m, Ord e) => a Int e -> e -> m Int
search ary val = do 
  (a,b) <- getBounds ary
  let {rec !low !high
    | low > high = return low
    | otherwise = do
      let { mid = (low + high) `div` 2; }
      p <- readArray ary mid
      if p < val then rec (mid + 1) high else rec low (mid - 1);}
  rec a b


solve :: Int -> [Int] -> Int
solve n ls = runST $ do
  ary <- newArray (0, n) inf :: ST s (STArray s Int Int)
  writeArray ary 0 0
  forM_ ls $ \x -> do
    ind <- search ary x
    el <- readArray ary ind
    writeArray ary ind (min el x)
  len <- search ary inf
  return $ n - len + 1

main = do
  cont <- getContents
  let (n : ls) = map read $ words cont in
    print $ solve n ls
