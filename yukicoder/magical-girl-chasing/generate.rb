sleep 1
x = 1 + rand(99)
y = x + 1 + rand(101 - x)
n = rand(80) + 1
puts "#{x} #{y}"
yes = rand(2) == 0
ary = [rand(20) + 1]
if yes
  cur = ary[0] * y / x + 1
  while ary.size < 80 && cur <= 100000
    ary[ary.size] = cur
    cur = (cur * y + x - 1) / x
  end
else
  for i in 1 ... n
    ary[i] = ary[i - 1] + rand(10) + 1
  end
end

puts ary.size
puts ary*' '
