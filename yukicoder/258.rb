gets;v=gets.split;d=k=0;e=l=[]
v.size.times{|i|u=k+v[i].to_i;d,e,k,l=u>d ?[u,l+[i+1],d,e]:[d,e]*2}
p d;puts e*' '
