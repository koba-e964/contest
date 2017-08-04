gets
s = gets.chomp.split
ma = 0
for v in s
  cnt = 0
  for c in v.split('')
    if c.upcase == c
      cnt += 1
    end
  end
  ma = [ma, cnt].max
end
puts ma
