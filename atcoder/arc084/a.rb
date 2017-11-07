a = []
for _ in 0 .. 1
  a << gets
end
ok = true
for j in 0 .. 2
  if a[0][j] != a[1][2 - j]
    ok = false
  end
end
puts ok ?'YES':'NO'
