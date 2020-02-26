n=gets.to_i
b=gets.split.map &:to_i
sa=b.inject(0, :+)

a=b.map{|v| sa-v*(n-1)}
puts a*' '
