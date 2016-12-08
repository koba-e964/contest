h,w=gets.split.map &:to_i
s=[]
for i in 0 ... h
  s[i]=gets.chomp
end
t = []
for i in 0 ... h
  for j in 0 ... w
    if s[i][j] == '*'
      t << [i, j]
    end
  end
end
for i in 0 ... h
  for j in 0 ... w
    if s[i][j] == '*'
      next
    end
    if (i - t[0][0]) * (j - t[1][1]) == (i - t[1][0]) * (j - t[0][1])
      next
    end
    s[i][j] = '*'
    for i in 0 ... h
      puts s[i]
    end
    exit 0
  end
end
