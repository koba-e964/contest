def exgcd(x, y)
  if (y == 0)
    return [1, 0]
  end
  r = x % y;
  q = x / y;
  s = exgcd(y, r);
  return [s[1], s[0] - q * s[1]]
end

def gcd(x, y)
  return y == 0 ? x : gcd(y, x % y)
end

def solve2(a1, b, a2, v)
  g = gcd(a1, a2);
  if (g == 0 || b % g != 0)
    return 0
  end
  t = exgcd(a1, a2)
  t[0] *= -b / g;
  t[1] *= b / g;
  bias = [t[0] / a2, t[1] / a1].min
  t[0] -= bias * a2;
  t[1] -= bias * a1;
  st = a2 * t[1]
  lcm = a1 / g * a2
  if st <= v
    return (v - st) / lcm + 1
  else
    return 0
  end
end

def solve(a1, b, a2, l, r) 
  return solve2(a1, b, a2, r) - solve2(a1, b, a2, l - 1)
end


a1, b1, a2, b2, l, r = gets.split.map &:to_i
puts (b1 >= b2 ? solve(a1, b1 - b2, a2, l - b2, r - b2) : solve(a2, b2 - b1, a1, l - b1, r - b1))
