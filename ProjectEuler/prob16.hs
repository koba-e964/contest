digitSum ::Integer->Integer
digitSum n|n<0 = 0
	|otherwise = sub n 0 where
		sub n s|n `seq`s`seq`False=undefined
		sub 0 s=s
		sub n s=sub (n `div` 10) (s+n`mod`10)

main=print(digitSum(2^1000))