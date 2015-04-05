main=do n<-readLn
	print (sum [x|x<-[1..n], x `mod` 3==0 || x `mod` 5==0])
