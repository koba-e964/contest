def solve_inner(x)
  if x <= 10
    return 0
  end
  r = x / 10 * 10 
  cnt = 10 * solve_inner(x / 10 - 1)
  cnt += (r - 1 + 88) / 100
  for i in r .. x
    s = i.to_s
    v = 0
    while v
      v = s.index('12', v)
      if v
        cnt += 1
        v += 1
      end
    end
  end
  cnt
end
def solve(x)
  cnt = solve_inner(x)
  v = 2
  while v <= x
    if v == 2
      cnt += 1
    else
      # count '...2's in [v.. v * 3 / 2)
      w = [v * 3 / 2 - 1, x].min - v
      if w >= 0
        cnt += (w + 8) / 10
      end
    end
    v *= 10
  end
  cnt
end

a, b = gets.split.map &:to_i
cnt = solve(b) - solve(a - 1)
if (a.to_s[0] == '2' && (a-1).to_s[-1] == '1')
  cnt -= 1
end
puts cnt
