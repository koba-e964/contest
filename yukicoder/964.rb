n=gets.to_i
for _ in 1..n
  lo=[1,10-n].min
  hi=lo+n-1
  for i in (lo..hi).map{|v| v}.reverse
    print i
  end
end
puts
