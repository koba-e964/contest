q=gets.to_i
for _ in 1..q
  a,b=gets.split.map &:to_i
  puts b+9*a-9
end
