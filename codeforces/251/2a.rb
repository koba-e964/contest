n,d=gets.split.map &:to_i
t=gets.split.map &:to_i
rem=d-10*(t.size-1)
for a in t
  rem-=a
end
c=2*(t.size-1)
if rem<0
  puts (-1)
  exit
end
puts rem/5+c
