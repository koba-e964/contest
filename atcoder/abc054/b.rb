n, m = gets.split.map &:to_i
a = []
b = []
for _ in 1 .. n
  a << gets.chomp
end
for _ in 1 .. m
  b << gets.chomp
end

for i in 0 .. n - m
  for j in 0 .. n - m
    # check match
    ok = true
    for k in 0 ... m
      if a[i + k][j .. j + m - 1] != b[k]
        ok = false
      end
    end
    if ok
      puts "Yes"
      exit 0
    end
  end
end
puts "No"
