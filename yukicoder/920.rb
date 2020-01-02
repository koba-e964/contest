a,b,c=gets.split.map &:to_i
a,b=[a,b].sort
while c>0
    if a<b; a+=1;else;b+=1;end
    c-=1
end
puts [a,b].min
