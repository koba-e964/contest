#!ruby -an
n,l=$F;l&&puts(n.to_i%-~l.to_i==1?:Lose:"Win")
