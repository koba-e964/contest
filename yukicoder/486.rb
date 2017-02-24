s = gets.chomp
a = s.index("OOO")
b = s.index("XXX")
if a.nil? && b.nil?
  puts "NA"
  exit 0
end
a = a || 1000
b = b || 1000
if a < b
  puts "East"
else
  puts "West"
end
