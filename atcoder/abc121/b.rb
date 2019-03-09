n,m,c=gets.split.map &:to_i
b=gets.split.map &:to_i
a = []
ans=0
for i in 1..n
  u=gets.split.map &:to_i
  tot = c
  for j in 0...m
    tot+=u[j]*b[j]
  end
  if tot>0;ans+=1;end
end
puts ans

