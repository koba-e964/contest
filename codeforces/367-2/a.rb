a,b = gets.split.map &:to_f
n = gets.to_i
puts (1..n).map{|_|
  x,y,v=gets.split.map &:to_f
  Math.sqrt((x-a)**2+(y-b)**2)/v
}.min
