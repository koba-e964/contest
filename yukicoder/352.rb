s=1
(2..gets.to_i).map{|i|s+=2.0/i+(i-2)*(i-1/3.0)/4}
p s
