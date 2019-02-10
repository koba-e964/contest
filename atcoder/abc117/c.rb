n,m=gets.split.map &:to_i
x=gets.split.map &:to_i
x.sort!
a=[]
for i in 0 .. m - 2
  a << x[i + 1] - x[i]
end
a.sort!
ans = 0
for i in 0 ... [0, m - n].max
  ans += a[i]
end
puts ans
