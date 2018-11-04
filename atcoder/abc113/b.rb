n=gets.to_i
t,a=gets.split.map &:to_i
h=gets.split.map &:to_i
mi=[10**8,1]
for i in 0 ... h.size
  v = h[i]
  d = (1000 * t - 6 * v - 1000 * a).abs
  mi = [mi, [d, i]].min
end
puts mi[1]+1
