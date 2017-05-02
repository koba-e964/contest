def parse(s)
  bias = 1
  if s.length <= 3 || s[-4] == '.'
    bias = 100
  end
  s.gsub!(/\./, "")
  bias * s.to_i
end

def enchant(n)
  s = ""
  if n % 100 != 0
    s = format(".%02d", n % 100)
  end
  n /= 100
  n = n.to_s
  nz = n.length
  for i in 0 .. nz - 1
    s = n[nz - 1 - i] + s
    if i < nz - 1 && i % 3 == 2
      s = "." + s
    end
  end
  s
end

s = gets.chomp
s = s.split(/[a-z]+/).select{|v| v != ""}
tot = 0
for v in s
  tot += parse(v)
end

puts enchant(tot)
