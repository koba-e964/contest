n=gets.to_i
d=[]
for i in 0 ... n
  d << gets.to_i
end
puts d.uniq.size
