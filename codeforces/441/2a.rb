n = gets.to_i
a = gets.to_i
b = gets.to_i
c = gets.to_i
n -= 1
x = [a, b, c].min
tot = 0
if n >= 1
  tot += [a, b].min
  tot += (n - 1) * x
end
puts tot
