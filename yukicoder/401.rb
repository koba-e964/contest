n=$<.read.to_i
for i in 0..n-1
  for j in 1..n
    x=if i < n/2 then i else n-1-i end
    printf("%03d",x)
    if j<n then print " " end
  end
  puts
end
