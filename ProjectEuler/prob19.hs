mw=[0,3,3,6,1,4,6,2,5,0,3,5]
mwl=[0,3,4,0,2,5,0,3,6,1,4,6]

xor True x=x
xor False x=not x
infixl 3 `xor`

ylc n k=let leap=n`mod`4==0 `xor` n`mod`100==0 `xor` n`mod`400==0 in
	length (filter (\x -> (x+k)`mod`7==0) (if leap then mwl else mw)) :ylc (n+1) (k+if leap then 2 else 1)

main= (print . sum . take 100) (ylc 1901 2)