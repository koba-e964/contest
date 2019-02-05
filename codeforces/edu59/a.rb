t=gets.to_i
for _ in 1..t
  n=gets.to_i
  s=gets.chomp
  if n==2&&s[0]>=s[1]
    puts 'NO'
  else
    puts 'YES'
    puts 2
    puts [s[0], s[1..-1]]*' '
  end
end
