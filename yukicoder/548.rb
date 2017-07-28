def kokushi?(str)
  count = {}
  for s in str.split('')
    if count[s].nil?
      count[s] = 0
    end
    count[s] += 1
  end
  two = 0
  for c in 'a' .. 'm'
    if count[c].nil?
      return false
    end
    if count[c] >= 3
      return false
    end
    if count[c] == 2
      two += 1
    end
  end
  two == 1
end

s = gets.chomp
impos = true
for c in 'a' .. 'm'
  if kokushi?(s + c)
    puts c
    impos = false
  end
end
if impos
  puts 'Impossible'
end

