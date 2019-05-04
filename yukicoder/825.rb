a,b,c = gets.split.map &:to_i
mi = 1000000
for i in 0..a
  for j in 0..b
    if i + j == 0; next; end
    for k in 0..i + 10 * j
      l = (k / 10) + (k % 10)
      if c == l + (a - i) + (b - j)
        mi = [mi, i + 10 * j - k].min
      end
    end
  end
end
if mi >= 1000000
  puts 'Impossible'
else
  puts mi
end

