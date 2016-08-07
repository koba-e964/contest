n,*c=$<.read.split.map &:to_i;l=c.reduce(:+)/(n-1);puts"#{c.count(l-2)} #{c.count(l-4)}"
