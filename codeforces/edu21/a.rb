n = gets.to_i
cand = []
for i in 0 .. 9
  for j in 1 .. 9
    cand << j * 10 ** i
  end
end

for v in cand
  if n < v
    puts v - n
    exit 0
  end
end
exit 1
