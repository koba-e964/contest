n, m, k = gets.split.map(&:to_i)
k -= 1
lr = k % 2
r = k / (2 * m)
d = (k % (2 * m)) / 2
puts "#{r + 1} #{d + 1} #{['L','R'][lr]}"
