s=gets.chomp
r=s.reverse
n=s.size
k=s[0..(n-3)/2]
if r==s && k.reverse == k
  puts'Yes'
else
  puts 'No'
end
