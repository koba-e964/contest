s = gets.chomp
ok = true
if s[0] != 'A'
  ok = false
end
u = (s[2..-2].split("")).sort
if u[0] != 'C'
  ok = false
end
u << s[1]
u << s[-1]
for i in 1 .. u.size - 1
  if u[i] <= 'Z'
    ok = false
  end
end
puts (ok ? 'AC' : 'WA')
