str = gets.chomp
ad, b = str.split('e')
a, d = ad.split('.')

b = b.to_i

if d.size > b
  intp = "#{a}#{d[0...b]}".to_i(10)
  frac = d[b..-1]
  if frac.to_i == 0
    puts "#{intp}"
  else
    puts "#{intp}.#{d[b..-1]}"
  end
else
  intp = ("#{a}#{d[0..-1]}" + '0' * (b - d.size)).to_i(10)
  puts "#{intp}"
end

