s = gets.chomp
n = s.length
cnt = 0
for i in 0 ... n / 2
  if s[i] != s[n - i - 1]
    cnt += 1
  end
end

puts (if cnt == 1 || (cnt == 0 && n % 2 == 1) then "YES" else "NO" end)
