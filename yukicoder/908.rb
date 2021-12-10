s=gets.chomp
ok = true
for i in 0...s.size
  if (s[i] == ' ') == (i % 2 == 0)
    ok = false
  end
end
puts ok ?'Yes':'No'
