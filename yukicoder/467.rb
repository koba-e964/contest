n = gets.to_i
d = gets.split.map &:to_i
x, y = gets.split.map &:to_i
z = [x.abs, y.abs].max
dmax = d.max
res = (z + dmax - 1) / dmax
if res == 1 && ! d.include?(z)
  puts 2
else
  puts res
end
