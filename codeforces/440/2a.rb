n, m = gets.split.map(&:to_i)
a = gets.split
b = gets.split
for i in 1 .. 99
  oka = false
  okb = false
  for c in i.to_s.split('')
    if a.include?(c)
      oka = true
    end
    if b.include?(c)
      okb = true
    end
  end
  if oka && okb
    puts i
    exit 0
  end
end
