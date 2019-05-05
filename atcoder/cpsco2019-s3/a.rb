s = gets.chomp
s.gsub!('A', 'x')
s.gsub!('O', 'A')
s.gsub!('x', 'O')
puts s
