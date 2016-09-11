a=gets.chomp
b=gets.chomp
c=gets.chomp
t=[a,b,c]
cur = 0
while 1
  if t[cur].size == 0
    puts [:A,:B,:C][cur]
    exit
  end
  ch = t[cur][0]
  t[cur] = t[cur].slice(1,t[cur].size)
  cur = ch.ord - 97
end
