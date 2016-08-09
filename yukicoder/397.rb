gets
b=u=[]
gets.split.map{|q|v=q.to_i;s=b.size;i=b.index{|k|k>v}||s;u+=(-s...-i).map{|t|[~t,-t]*' '};b[i...i]=v;};p u.size;puts u
