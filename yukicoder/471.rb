m=gets.to_i
x=Array.new(m)
y=Array.new(m)
(0 ... m).each {|i|
  x[i], y[i] = gets.split.map &:to_i
}
puts "? 0 0"
STDOUT.flush
ox, oy = gets.split.map &:to_i
puts "? 1 0"
STDOUT.flush
kx, ky = gets.split.map &:to_i
kx -= ox
ky -= oy
puts "? 0 1"
STDOUT.flush
lx, ly = gets.split.map &:to_i
lx -= ox
ly -= oy

puts "!"
(0 ... m).each {|i|
  print ox + kx * x[i] + lx * y[i]
  print " "
  puts oy + ky * x[i] + ly * y[i]
}
