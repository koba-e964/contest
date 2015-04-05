digits :: Int->[Int]
digits 0=[]
digits n=n `mod` 10:digits (n `div` 10)

palin n=reverse ls==ls where ls=digits n
main=print (maximum [x*y|k<-sequence [[100..999],[100..999]], let x:y:_=k,palin (x*y)])
