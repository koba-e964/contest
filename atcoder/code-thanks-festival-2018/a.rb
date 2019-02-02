t,a,b,c,d=gets.split.map &:to_i
if a+c<=t
  puts b+d
elsif c<=t
  puts d
elsif a<=t
  puts b
else
  puts 0
end
