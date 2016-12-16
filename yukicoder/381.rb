n=gets.to_i
c=0
(0 .. 3340000).each {|i|
  c += n[i]
}
puts c
