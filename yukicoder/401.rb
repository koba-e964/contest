n=gets.to_i
ary = Array.new(n){|v| Array.new(n)}
cnt = 1
for i in 0..(n + 1)/2
  ulim = n - 1 - i
  # [i, ulim] * [i, ulim]
  for j in i .. ulim
    ary[i][j] = cnt
    cnt += 1
  end
  for j in (i + 1) .. ulim
    ary[j][ulim] = cnt
    cnt += 1
  end
  for j in (i .. (ulim - 1)).to_a.reverse
    ary[ulim][j] = cnt
    cnt += 1
  end
  for j in ((i + 1) .. (ulim - 1)).to_a.reverse
    ary[j][i] = cnt
    cnt += 1
  end
end
for i in 0 ... n
  for j in 0 ... n
    printf "%03d", ary[i][j]
    if j != n - 1
      print " "
    end
  end
  puts
end

