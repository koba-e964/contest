x = gets.to_f
n = 100000
ary = Array.new(n + 1)
for i in 1 .. n
  ary[n - i] = 1.0 / (x + i) / (x + i)
end
ary[n] = 1.0 / (x + n + 0.5)
ary.sort!
sum = 0
for a in ary
  sum += a
end
p sum
