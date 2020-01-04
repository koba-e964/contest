n=gets.to_i
c=0
m=n
while n>1
  if n%2==0
    n/=2
  else
    n=3*n+1
  end
  c+=1
  m=[m,n].max
end
puts c
puts m
