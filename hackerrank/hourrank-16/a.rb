#!/bin/ruby

n = gets.strip.to_i
c = gets.strip
c = c.split(' ').map(&:to_i)
c.sort!
mi = c[0]
mic = 0
for i in 0 ... n
    if mi == c[i]
        mic += 1
    end
end
if n == 1
    puts (c[0] * 2).to_s + " 1"
elsif mic == 1
    puts [c[0] * 2, c[1]].min.to_s + " 1"
else
    puts c[0].to_s + " " + n.to_s
end
