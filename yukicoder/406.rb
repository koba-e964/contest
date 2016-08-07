n=gets.to_i;c=gets.split.map(&:to_i).sort
puts(if(0..n-2).find{|v|c[v+1]-c[v]!=c[1]-c[0]}||c[1]==c[0]then"NO"else"YES"end)
