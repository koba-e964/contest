a,b=gets.split.map &:to_f
p,q,r=gets.split.map &:to_f
d=b*p-(b-a)*q
puts d/(q+r)+b
