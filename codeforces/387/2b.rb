def miss
  puts "==="
  exit 0
end

n=gets.to_i
s=gets.chomp
if n % 4 != 0
  miss
end
inv_t={'A'=>0,'G'=>1,'C'=>2,'T'=>3,'?'=>4}
t='AGCT?'.split('')
freq=Array.new(5, 0)
s.split('').each {|c|
  freq[inv_t[c]] += 1
}
(0 .. 3).each {|i|
  if freq[i] > n / 4
    miss
  end
}
(0 ... n).each {|i|
  if s[i] == '?'
    for j in 0 .. 3
      if freq[j] < n / 4
        s[i] = t[j]
        freq[j] += 1
        break
      end
    end
  end
}
puts s
