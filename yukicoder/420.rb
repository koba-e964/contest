def c(a,b,s)(1..b).each{|i|s*=a-i;s/=i};b<0?0:s;end
x=gets.to_i
puts x<99?[c(32,x,1),c(31,x-1,2**31-1)]*' ':'0 0'
