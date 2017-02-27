n = gets.to_i
a = gets.split.map &:to_i
b = gets.split.map &:to_i
af = Array.new(5, 0)
bf = Array.new(5, 0)
for v in a
  af[v - 1] += 1
end
for v in b
  bf[v - 1] += 1
end

am = 0
bm = 0

for i in 0 .. 4
  u = af[i]
  v = bf[i]
  if (u + v) % 2 == 1
    puts -1
    exit 0
  end
  if u > v
    am += (u - v) / 2
  else
    bm += (v - u) / 2
  end
end
puts [am, bm].max
