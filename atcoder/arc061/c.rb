a=gets.chomp.reverse
w=[1,11,112,1124,11248,112496,1124992,11249984,112499968,1124999936]
s=(0...a.size).map{|v|w[v]*(a[v].ord-48)*2**(a.size-v-1)}.reduce :+
p s
