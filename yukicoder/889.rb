require 'prime'

n=gets.to_i
if Prime::prime?(n)
  puts 'Sosu!'
  exit
end
for i in 2..8
  if i*i==n
    puts 'Heihosu!'
    exit
  end
  if i*i*i==n
    puts 'Ripposu!'
    exit
  end
end
if n==6||n==28
  puts 'Kanzensu!'
  exit
end
puts n
