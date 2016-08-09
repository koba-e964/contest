def f(n,d)g=n.gcd(d);[n/g,d/g]end
t,u,v=$<.map &:to_i
a,b=f(t-u,t*u)
c,d=f(u-v,u*v)
puts [b.lcm(d),a.gcd(c)]*'/'
