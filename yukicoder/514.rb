M = 10 ** 9 + 1
puts "0 0"
STDOUT.flush
d1 = gets.to_i
puts M.to_s + " 0"
STDOUT.flush
d2 = gets.to_i
x = (d1 - d2 + M) / 2
y = d1 - x
puts "#{x} #{y}"
STDOUT.flush
gets
