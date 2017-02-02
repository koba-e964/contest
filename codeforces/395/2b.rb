n=gets.to_i
a=gets.split.map &:to_i
for i in 0 ... n / 2
  if i % 2 == 0
    tmp = a[i]
    a[i] = a[n - 1 - i]
    a[n - 1 - i] = tmp
  end
end
puts a*' '
