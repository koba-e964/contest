discLog::Int->Int->Int

discLog a m
  |m<=0 = undefined
  |gcd a m /= 1 = 0
  |otherwise = sub a 1 where
    sub 1 k=k
    sub b k = sub (a*b `mod` m) (k+1)

main=print (maximum [(discLog 10 x,x)|x<-[2..999]])