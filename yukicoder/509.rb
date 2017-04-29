n = gets.chomp
tot = 2
tot2 = 1
qq  = [3,1,1,1,3,1,3,1,5,3]
qq2 = [3,2,2,2,3,2,3,2,4,3]
for c in n.split('')
  tot += qq[c.ord - 48]
  tot2 += qq2[c.ord - 48]
end
tot2 = [tot2, tot].min
puts tot2
