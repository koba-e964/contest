t=gets.to_i
for case_nr in 1..t
  n=gets.to_i
  a=[]
  tr=0
  for _ in 1..n
    v=gets.split.map &:to_i
    a<<v
  end
  for i in 0...n
    tr+=a[i][i]
  end
  r=0
  c=0
  for i in 0...n
    v=a[i].clone
    v.uniq!
    if v.size != n
      r += 1
    end
  end
  for i in 0...n
    v=[0]*n
    for j in 0...n
      v[j]=a[j][i]
    end
    v.uniq!
    if v.size != n
      c += 1
    end
  end
  puts "Case \##{case_nr}: #{tr} #{r} #{c}"
end
