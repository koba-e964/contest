n = gets.to_i
ary = []
x = 50000
while n > 0
  y = x * x
  while n >= y
    n -= y
    ary << x
  end
  x -= 1
end
for i in 0 ... ary.size
  print((['iw', 'co', 'sa'][i % 3] * ary[i])[0..-2])
end
puts()
