c,d=gets.split.map &:to_f
a=(20*c-8*d)/13
b=7*(3*d-c)/13
if a<0
  puts 7000*c
  exit 0
end
if b<0
  puts 4000*d
  exit 0
end
puts 1000*(a+2*b)
