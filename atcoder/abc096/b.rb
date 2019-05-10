a=gets.split.map &:to_i
a.sort!
k=STDIN.read.to_i
for _ in 1..k
  a[-1]*=2
end
puts a[0]+a[1]+a[2]
