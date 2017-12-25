n, k, x = gets.split.map(&:to_i)
a = gets.split.map(&:to_i)
for i in 1 .. k
  a[-i] = [a[-i], x].min
end
puts a.reduce(:+)
