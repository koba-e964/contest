fac = (5 ** 47 - 1) / 4
m = gets.to_i
if m % 2 == 0
  puts "odd"
  exit 0
end
puts ((fac % m) % 2 == 0 ? "even" : "odd")
