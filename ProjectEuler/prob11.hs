import Control.Applicative
import Data.Maybe

numbers :: Int->IO [[Int]]
numbers 0=return $! []
numbers n|n<0=undefined
	|otherwise=do
		(:) <$> (map read . words <$> getLine) <*> numbers (n-1)

searchMax :: Int->Int->Int->Int->[[Int]]->Int
searchMax m n i j ary|i==n =m
	|j==n =searchMax m n (i+1) 0 ary
	|otherwise = let {mm=maximum $ 0: do{
			(d1,d2)<-[(0,1),(1,0),(1,1),(1,-1)];
			maybeToList(foldr
				((\p q->(*) <$> p <*> q)::Maybe Int->Maybe Int->Maybe Int)
				(Just 1)
				(map (\x->maybeAt ary (i+x*d1) (j+x*d2)) [0..3])
			);
			}}in searchMax (max mm m) n i (j+1) ary;
maybeAt :: [[a]]->Int->Int->Maybe a
maybeAt lss i j| i<0 || i>= length lss = Nothing
		|j<0 || j>= length (lss!!i) =Nothing
		|otherwise = Just ((lss!!i)!!j)

main=do
	num<-numbers 20
	print (searchMax 0 20 0 0 num)
	return ()
	
