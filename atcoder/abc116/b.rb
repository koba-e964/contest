s=gets.to_i
hash={}
cnt=0
while 1
  cnt+=1
  if hash[s]
    puts cnt
    exit
  end
  hash[s]=cnt
  if s%2==0
    s/=2
  else
    s=3*s+1
  end
end
  
