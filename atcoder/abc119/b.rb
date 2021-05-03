n = gets.to_i
sum = 0
for i in 0...n
  x, y = gets.split
  x = x.to_f
  if y == 'BTC'
    x *= 380000
  end
  sum += x
end
puts sum
       
