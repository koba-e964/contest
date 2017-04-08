n, d = gets.split.map(&:to_i)
if d <= n
  puts "A" * d + "C" * (n - d)
  exit 0
end
puts "A" * (2 * n - d) + "B" * (d - n)
