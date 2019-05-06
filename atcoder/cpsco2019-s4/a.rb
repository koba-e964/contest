l,x=gets.split.map &:to_i
x%=2*l
if x>=l
  x=2*l-x
end
puts x
