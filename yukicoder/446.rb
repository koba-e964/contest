a=gets.chomp
b=gets.chomp
na=a.to_i
nb=b.to_i
if na.to_s!=a||nb.to_s!=b||na>12345||nb>12345
  puts "NG"
else
  puts "OK"
end
