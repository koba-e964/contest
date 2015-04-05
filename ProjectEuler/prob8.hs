import Data.Char

consMax :: [Int]->Int->Int
consMax ls m|length ls<=4 = m
	    |otherwise = consMax (tail ls) (max m (foldr (*) 1 (take 5 ls)))

digitsToInts :: [Char]->[Int]
digitsToInts = map digitToInt

main = do
	x<-getLine;
	print (consMax (digitsToInts x) 0)
