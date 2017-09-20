s = gets.chomp
c = 'A'
cnt = 0
for v in s.split('')
  if v <= c
    cnt += 1
  end
  c = v
end
puts cnt
