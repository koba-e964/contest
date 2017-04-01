s = gets.chomp
a = Array.new(5, 0)
tbl = {'Y' => 0, 'E' => 1, 'A' => 2, 'H' => 3, '!' => 4}
for i in 0 ... s.length
  c = s[i]
  a[tbl[c]] += 1
end
for i in 0 .. 4
  print a[i]
  if i == 4
    puts
  else
    print " "
  end
end
