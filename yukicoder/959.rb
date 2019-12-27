n, m = gets.split.map &:to_i
p = gets.to_f
if n == 1
  if m == 1
    puts p
  else
    puts p*(2*p+p*p*(m-2))
  end
  exit
end
if m == 1
  puts p*(2*p+p*p*(n-2))
  exit
end
puts p*p*p*(4+p*(2*n+2*m-8)+p*p*(n-2)*(m-2))
