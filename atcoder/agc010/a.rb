n = gets.to_i
a = gets.split.map &:to_i
h = 0
for i in 0 ... n
  h += if a[i] % 2 == 1 then 1 else 0 end
end
puts (if h % 2 == 0 then "YES" else "NO" end)
