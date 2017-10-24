s = gets
names = ["Danil", "Olya", "Slava", "Ann", "Nikita"]
cnt = 0
for n in names
  l = s.index(n)
  r = s.rindex(n)
  if l
    if l == r
      cnt += 1
    else
      cnt += 2
    end
  end
end
puts cnt == 1 ? "YES" : "NO"
