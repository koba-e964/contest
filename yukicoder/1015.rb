n,x,y,z=gets.split.map &:to_i
a=gets.split.map &:to_i

b = [x, y, z]

c = []

for v in a
  v = v/1000 + 1
  b[2] -= v / 10
  c << (v % 10)
end

r = [0, [-b[2], b[1] / 2].min].max
b[2] += r
b[1] -= 2 * r
if b[2] < 0 && b[1] == 1 && b[0] >= 5
  b[2] += 1
  b[1] = 0
  b[0] -= 5
end
r = [0, [-b[2], b[0] / 10].min].max
b[2] += r
b[0] -= 10 * r

if b[2] < 0
  puts 'No'
  exit
end
c.sort!

r = [c.size, b[2]].min
c = c[0...c.size - r]
b[2] -= r

d = []

for v in c
  if v >= 5
    b[1] -= 1
    v -= 5
  end
  d << v
end

r = [0, [-b[1], b[0] / 5].min].max
b[1] += r
b[0] -= 5 * r
if b[1] < 0
  puts 'No'
  exit
end

d.sort!
r = [d.size, b[1]].min
d = d[0...d.size - r]
b[1] -= r

s = d.sum
if b[0] < s
  puts 'No'
else
  puts 'Yes'
end
