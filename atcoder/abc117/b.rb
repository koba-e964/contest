n=gets.to_i
l=gets.split.map &:to_i
l.sort!
s=0
for i in 0..n-2
  s+=l[i]
end
puts l[n-1]<s ?'Yes':'No'

