n = gets.to_i
for i in 0 ... n
    puts (0 ... n).map{|j|(2*i-n+1).abs+(2*j-n+1).abs<=n-1?"D":"*"}*''
end
