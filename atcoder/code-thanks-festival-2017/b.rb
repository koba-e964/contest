a=gets.chomp
for i in 0...a.size
  b = a[i + 1..-1]
  if b.size > i + 1
    next
  end
  if b.size <= i && b.reverse == a[i - b.size...i]
    puts i-b.size
    exit
  end
  if b.reverse == a[i + 1 - b.size...i + 1]
    puts i-b.size+1
    exit
  end
end
