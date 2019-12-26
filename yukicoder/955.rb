a,b,c=gets.split.map &:to_i
if a==0
  if b==0
    if c==0
      puts (-1)
    else
      puts 0
    end
    exit
  end
  puts 1
  printf "%.15f\n", (-c/1.0/b)
  exit
end

if a<0
  a=-a
  b=-b
  c=-c
end
d=b*b-4*a*c
if d==0
  puts 1
  printf "%.15f\n", (-b/2.0/a)
  exit
end
if d<0
  puts 0
  exit
end
puts 2
s=Math.sqrt(d)/(2.0*a).abs
t=Math.sqrt(d)
if b >= 0
  printf "%.15f\n", (-b/(2.0*a)-s)
else
  printf "%.15f\n", (2.0*c/(-b+t))
end
if b <= 0
  printf "%.15f\n", (-b/(2.0*a)+s)
else
  printf "%.15f\n", (2.0*c/(-b-t))
end
