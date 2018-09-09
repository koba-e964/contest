n=gets.to_i
w=[]
for i in 0 ... n
  w << gets.chomp
end

ok = true
for i in 0 ... n
  for j in 0 ... i
    if w[i] == w[j]
      ok = false
    end
  end
end
for i in 0 ... n - 1
  if w[i][-1] != w[i+1][0]
    ok = false
  end
end
puts ok ?"Yes":"No"
