n = gets.to_i
for _ in 0 ... n
  x, y, p, q = gets.split.map(&:to_i)
  y -= x
  q -= p
  if p == 0 || q == 0
    if (x == 0 && p == 0) || (y == 0 && q == 0)
      puts 0
    else
      puts(-1)
    end
    next
  end
  a1 = (x + p - 1) / p
  a2 = (y + q - 1) / q
  a = [a1, a2].max
  r1 = a * p - x
  r2 = a * q - y
  puts r1 + r2
end

