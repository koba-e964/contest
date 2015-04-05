import Control.Applicative
import Data.Array.ST
import Control.Monad.ST
import Control.Monad
readInts:: Int->IO [[Int]]
readInts 0=return []
readInts n|n<0=undefined
	|otherwise = (:) <$> (map read . words <$> getLine) <*> readInts (n-1)

solve :: Int->[[Int]]->Int
solve n dat=runST $ do
	ary<- newArray ((0,0),(n,n)) 0 ::ST s(STArray s (Int,Int) Int)
	writeArray ary (0,0) (dat!!0!!0)
	forM_ [1..n-1] (\i->do{
		x<-readArray ary (i-1,0);
		writeArray ary (i,0) (x+dat!!i!!0);
		y<-readArray ary (i-1,i-1);
		writeArray ary (i,i) (y+dat!!i!!i);
		forM_ [1..i-1] (\j-> do{
			z<-readArray ary (i-1,j-1);
			w<-readArray ary (i-1,j);
			writeArray ary (i,j) ((max z w)+(dat !! i !! j));
		});
	})
	maximum <$> (forM [0..n-1] (\i->readArray ary (n-1,i)))

main=do
	dat<-readInts 15
	print (solve 15 dat)
	