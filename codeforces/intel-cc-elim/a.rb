kind = gets.to_i
time = gets.chomp
if time[3] >= '6'
  time[3] = '0'
end

poss = []
if kind == 24
  poss = (0 .. 23).map{|v| format("%02d", v)}
else
  poss = (1 .. 12).map{|v| format("%02d", v)}
end

mi = 100
mini = "indigo"

for p in poss
  s = time[0 .. 1]
  cnt = 0
  for i in 0 .. 1
    cnt += p[i] == s[i] ? 0 : 1
  end
  if mi > cnt
    mi = cnt
    mini = p
  end
end
time[0] = mini[0]
time[1] = mini[1]
puts time
