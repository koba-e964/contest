sq x=x*x

main=do n<-readLn
	print (sq (sum [1..n])- sum (map sq [1..n]))

