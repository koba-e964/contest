isqrt=floor . sqrt . fromIntegral

isPrime n|n<=1 = False
	|otherwise = all (\x -> n `mod` x/=0) [2..isqrt n]
nthPrime 0 n=n-1
nthPrime k p|isPrime p=nthPrime (k-1) (p+1)
	    |otherwise=nthPrime k (p+1)

main=do x<-readLn
	print (nthPrime x 2)

