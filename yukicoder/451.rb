n=gets.to_i
b=Array.new(n)
for i in 0...n
  b[i]=gets.to_i
end
a=Array.new(n+1)
a[0]=0
for i in 0...n
  if i%2==0
    a[i+1]=b[i]-a[i]
  else
    a[i+1]=a[i]-b[i]
  end
end
lim=10**18
xl=1-a[0]
xh=lim-a[0]
yl=a[1]-lim
yh=a[1]-1
for i in 0..n
  if i%4==0||i%4==3
    xl=[1-a[i],xl].max
    xh=[lim-a[i],xh].min
  else
    yl=[a[i]-lim,yl].max
    yh=[a[i]-1,yh].min
  end
end
xl=[xl,yl].max
xh=[xh,yh].min
if xl>xh
  puts -1
else
  puts n+1
  for i in 0..n
    puts a[i]+(if i%4==0||i%4==3 then xl else -xl end)
  end
end
