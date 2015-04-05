import Data.List
fib::[Integer]
fib=0:1:zipWith (+) fib (tail fib)

main=print(find (\x->length (show x)>=1000) fib)