n=gets.to_i
s=[]
for i in 1..n
  t=gets.chomp
  s << t
end
ans = 0
for i in 1...n-1
  for j in 1...n-1
    if s[i][j]=='X'&&s[i-1][j-1]=='X'&&s[i+1][j-1]=='X'&&s[i-1][j+1]=='X'&&s[i+1][j+1]=='X'
      ans += 1
    end
  end
end
puts ans
