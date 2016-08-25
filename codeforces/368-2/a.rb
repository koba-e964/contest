n,_=gets.split
n = n.to_i
ok = false
for i in 1 .. n
  ok ||= gets.split.find{|v|['M', 'C', 'Y'].index(v)}
end
puts(ok ? "#Color" : "#Black&White")
