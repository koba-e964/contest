include Math
def bigf(n)
  num = 0.0
  den = 0.0
  for i in 1 .. n
    num += 0.81 ** i
    den += 0.9 ** i
  end
  sqrt(num) / den
end

def f(n)
  finf = bigf(400)
  1200.0 * (bigf(n) - finf) / (bigf(1) - finf)
end



n = gets.to_i
s = Array.new(n)
for i in 0 ... n
  s[i] = gets.to_f
end

num = 0.0
den = 0.0
for i in 1 .. n
  num += 2.0 ** (s[i - 1] / 800.0) * 0.9 ** i
  den += 0.9 ** i
end

rating = num / den
rating = 800.0 * log2(rating)
rating -= f(n)
puts rating.to_i
