s=gets.to_i
a=s/100
b=s%100
fl=0
if 1<=a&&a<=12
  fl|=1
end
if 1<=b&&b<=12
  fl|=2
end
puts ['NA', 'MMYY', 'YYMM', 'AMBIGUOUS'][fl]
