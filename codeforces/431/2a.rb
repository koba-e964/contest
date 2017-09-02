n = gets.to_i
a = gets.split.map(&:to_i)
if a.size % 2 == 1 && a[0] % 2 == 1 && a[-1] % 2 == 1
  puts "Yes"
else
  puts "No"
end

