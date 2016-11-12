def gcd(x,y)
  while y!=0
    r=x%y
    x=y
    y=r
  end
  x
end

ns=gets.chomp
n=ns.to_i
nsu=ns.chars.sort.uniq.map{|v| v.ord - 48}
g = 0
for t in nsu
  for u in nsu
    g = gcd(g, t - u)
  end
end
if g == 0
  puts ns
else
  puts gcd(n, 9 * g)
end

