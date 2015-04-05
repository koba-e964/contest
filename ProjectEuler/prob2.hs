fibl x y=if x>=4000000 then [] else x:fibl y (x+y)

main=print (sum [x|x<-fibl 1 2, x `mod` 2==0])
