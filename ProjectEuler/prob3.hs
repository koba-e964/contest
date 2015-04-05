isqrt :: Integral i=>i->Int
isqrt=floor . sqrt . fromIntegral
isPrime 2=True
isPrime n|n<=1 =False
isPrime n=all (\x -> n `mod` x==0) [2..isqrt n]

maxFactor n=mf n 2 1
	where {mf 1 _ k=k;
		mf n p k|n `mod` p==0 = mf (n `div` p) p (max p k)
			|n<p*p = max n k 
			|otherwise =mf n (p+1) k;
	}

main=do x<-readLn
	print (maxFactor x)
