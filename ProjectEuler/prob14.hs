import Data.Array
import Data.Array.ST
import Control.Applicative
import Control.Arrow
import Control.Monad
import Control.Monad.ST

collatz ::Integer->Integer->Integer
collatz count state
	|count `seq` state `seq` False=undefined
	|state<=1 = count
	|state `mod` 2==0 = collatz (count+1) (state `div` 2)
	|otherwise = collatz (count+2) ((3*state+1) `div` 2)


limit=1000000
memomax=1000000

memoizedCollatz :: STArray s Int Int->Integer->ST s Int

memoizedCollatz memo state=do
	bounds<-getBounds memo
	let{inBounds=inRange ((fromIntegral *** fromIntegral) bounds) state;}
	m<-if inBounds then readArray memo (fromIntegral state) else return (-1)
	if m>=0 then return $! m else do{v<-sub state;when inBounds (writeArray memo (fromIntegral state) v); return v;} where
		sub s
			|s<=1 = return 0
			|s `mod` 2 ==0 = (1+) <$> memoizedCollatz memo (s `div` 2)
			|otherwise = (1+) <$> memoizedCollatz memo (3*s+1)


getMax :: Integral i=>i->(i,Int)
getMax n= runST $ do
	memo<-newArray (0,memomax) (-1) :: ST s (STArray s Int Int)
	res<-forM [0..(fromIntegral n)] (memoizedCollatz memo)
	-- return memo
	return (maxid res 0 0 0)

maxid ::(Ord a,Integral i)=>[a]->i->i->a->(i,a)
maxid ls base m e|ls `seq` base `seq` m `seq` e `seq` False=undefined
maxid [] _ m e=(m,e)
maxid (x:xs) base m e=case compare x e of
			GT->maxid xs (base+1) base x
			_->maxid xs (base+1) m e

main=print(getMax limit)
