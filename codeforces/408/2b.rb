n, _, k = gets.split.map(&:to_i)
h = gets.split.map(&:to_i)
ary = Array.new(n, false)
h.each {|v|
  ary[v - 1] = true;
}
cur = 0
k.times {
  if ary[cur]
    puts cur + 1
    exit 0
  end
  a, b = gets.split.map(&:to_i)
  a -= 1
  b -= 1
  if a == cur
    cur = b
  elsif cur == b
    cur = a
  end
}
puts cur + 1
