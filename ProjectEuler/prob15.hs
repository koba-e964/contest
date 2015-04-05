biCoef :: Integer->Integer->Integer
biCoef n k
	|k<0=0
	|otherwise = sub k 1 where
		sub 0 m=m
		sub x y=sub (x-1) (y*(n-x+1) `div` (k+1-x))

main=print(biCoef 40 20)
	