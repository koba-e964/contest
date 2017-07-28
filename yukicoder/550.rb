def bin_sect(a, b, c, lo, hi)
  while hi - lo > 1
    mid = (hi + lo) / 2
    val = mid * (mid * (mid + a) + b) + c
    if val > 0
      hi = mid
    else
      lo = mid
    end
  end
  lo
end


a, b, c = gets.split.map(&:to_i)
x0 = 0
if c > 0
  x0 = bin_sect(a, b, c, -10 ** 10, 0)
elsif c < 0
  x0 = bin_sect(a, b, c, 0, 10 ** 10)
end

d = a + x0
e = x0 == 0 ? b : -c / x0
ss = Math.sqrt(d * d - 4 * e).to_i
x1 = (-d + ss) / 2
x2 = (-d - ss) / 2
x = [x0, x1, x2].sort
puts x*' '
