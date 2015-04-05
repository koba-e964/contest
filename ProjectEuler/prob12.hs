factorize::Int->[(Int,Int)]

factorize 1=[]
factorize n
	|n<0 = undefined
	|otherwise = sub n 2 0 where
		sub 1 p 0=[]
		sub n p 0
			|n<p*p = [(n,1)]
			|n `mod` p /=0 = sub n (p+1) 0
		sub n p k
			|n `mod` p /=0 = (p,k):sub n (p+1) 0
			|otherwise = sub (n `div` p) p (k+1)


numDivisors :: Int->Int
numDivisors n=let x=factorize n in
		foldr (*) 1 (map ((+ 1).snd) x)

limit=100000000
ans=500

calc n|n `mod` 2==1 = (numDivisors n)*(numDivisors ((n+1)`div` 2)) 
	|otherwise =(numDivisors (n+1))*(numDivisors (n `div` 2)) 

solve n|n==limit = error "limit exceeded"
	|otherwise = if calc n>=ans then n else solve (n+1)

main=let x=solve 1 in print ("n="++show x++" triangle="++show (x*(x+1) `div` 2))
