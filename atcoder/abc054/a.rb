a, b= gets.split.map &:to_i
if a == 1
  a = 14
end
if b == 1
  b = 14
end
if a < b
  puts "Bob"
elsif a > b
  puts "Alice"
else
  puts "Draw"
end
