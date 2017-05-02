n, k = gets.split.map(&:to_i)
a = gets.split.map(&:to_i)

tot = 0
for i in 0 ... n - 1
  if a[i] + a[i + 1] < k
    add = k - a[i] - a[i + 1]
    a[i + 1] += add
    tot += add
  end
end
puts tot
puts a.join(' ')
