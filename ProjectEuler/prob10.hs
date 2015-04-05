import Data.Array.ST
import Data.Array
import Control.Monad


sieve n=runSTArray $ do
	ary <- newArray (2,n) True
	let{sub k|k*k>n = return ()
		 |otherwise= do t<-readArray ary k
				if t then forM_ [2*k,3*k..n] (\x -> writeArray ary x False) else return ()
				sub (k+1) 
	}in sub 2
	return ary

primes=map fst.filter snd.assocs.sieve

main=print (sum (primes 1999999))
