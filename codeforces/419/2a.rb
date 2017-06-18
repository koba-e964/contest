hh, mm = gets.chomp.split(':').map(&:to_i)
pal = []
for i in 0 .. 23
  if i % 10 <= 5
    pal << i * 60 + (i / 10) + 10 * (i % 10)
  end
end
pal << 1440
mi = 100000
t = 60 * hh + mm
for v in pal
  if v >= t
    mi = [mi, v - t].min
  end
end
puts mi
