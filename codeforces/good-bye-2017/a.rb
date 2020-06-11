s=gets.chomp
a='aeiou13579'
c=0
for ch in a.split('')
  c+=s.count(ch)
end
puts c
