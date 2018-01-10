k1 = gets.chomp
k2 = gets.chomp

map = {}
for i in 0 ... 26
  map[k1[i]] = k2[i]
end

s = gets.chomp
t = s.split('').map{|c|
  if c != c.downcase
    map[c.downcase].upcase
  else
    map[c] ? map[c] : c
  end
}.join()
puts t
