k = gets.to_i
if k > 36
  puts -1
else
  puts '8'*(k/2)+'4'*(k%2)
end
