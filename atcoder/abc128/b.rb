n=gets.to_i
tbl=[]
for i in 1..n
  s,p=gets.split
  p=p.to_i
  tbl << [s,-p,i]
end
tbl.sort!
for t in tbl
  puts t[2]
end
