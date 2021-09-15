gets
a=gets.split
if a.size == 3
  puts '"assert"'
  exit
end
a=[a[0].to_i]
for _ in 1..2
  a << gets.to_i
end
s=a.reduce(:+)
a.sort!
a.uniq!
puts s-a[1]
