a = gets.chop
b = gets.chop
if a == b
  puts -1
else
  puts [a.size, b.size].max
end
