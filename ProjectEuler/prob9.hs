isqrt=floor.sqrt.fromIntegral
isSq x|x<0=False
      |otherwise=x==y*y where y=isqrt x

enum c=[(a,b)|a<-[1..c `div` 2],let b=isqrt (c*c-a*a), isSq(c*c-a*a)]

solve sum=do
	c<-[1..sum `div` 2]
	(a,b)<-enum c
	if a+b+c==sum then return (a,b,c) else []

main= let (a,b,c):_=(solve 1000) in
	print (a*b*c)