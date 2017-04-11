n, m, k = gets.split.map(&:to_i)
m -= 1
a = gets.split.map(&:to_i)
mi = 2 ** 100
(0 ... n).each {|i|
  if a[i] <= k && a[i] != 0
    mi = [mi, (m - i).abs * 10].min
  end
}
puts mi
