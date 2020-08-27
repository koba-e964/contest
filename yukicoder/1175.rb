a,b,c,d,e,f=gets.split.map &:to_f
det=a*e-b*d
x=(c*e-b*f)/det
y=(a*f-c*d)/det
puts "#{x} #{y}"

