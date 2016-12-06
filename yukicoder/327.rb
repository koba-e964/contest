n = gets.to_i

cnt = 1
pw = 26
while n >= pw
  n -= pw
  cnt += 1
  pw *= 26
end
s = []
for i in 0 ... cnt
  s[cnt - 1 - i] = (65 + (n % 26)).chr
  n /= 26
end
puts s*''
