import Control.Applicative;

readLines 0=return []
readLines n|n<0=undefined
	|otherwise=do
		x<-getLine
		y<-readLines (n-1)
		return (x:y)

main=do
	is<-map read <$> readLines 100
	let str=show (sum is) in
		print (take 10 str)
