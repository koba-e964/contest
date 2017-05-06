def generate(filename, strs, queries)
  fp = open(filename, "w")
  fp.puts(strs.size)
  for s in strs
    fp.puts(s)
  end
  fp.puts(queries*' ')
  fp.close
end


srand(0x31415926)
# hard, 800 strings of length 1000 ==> 800000
generate("test-hard-0.txt", ["a" * 1000, "b" * 1000] * 400, [3000000, 1, 2])
generate("test-hard-1.txt", ["a" * 1998, "b" * 1998] * 200 + ["a"] * 800, [3000000, 1, 1000])
for i in 2 .. 3
  app = ""
  l = rand(998)
  app = "a" * l + "b" * (998 - l)
  generate("test-hard-#{i}.txt", ["a" * 1998, "a" * 1000 + app] * 200 + ["a"] * 800, [3000000, 1, 1000])
end

# easy cases: 100 strings of length 8000 ===> 800000
for i in 0 .. 3
  len = 100
  strlen = 8000
  ary = Array.new(len)
  for j in 0 ... len
    s = ""
    for k in 0 ... strlen
      s += (rand(26) + 10).to_s(36)
    end
    ary[j] = s
  end
  x = rand(len * (len - 1))
  d = rand(len * (len - 1))
  generate("test-easy-#{i}.txt", ary, [3000000, x, d])
end

# tough cases: 10 strings of length 80000 ===> 800000
# Might cause overflow of int32
generate("test-tough-0.txt", ["a" * 10000, "b" * 10000] * 40, [3000000, 1, 2])
generate("test-tough-1.txt", ["a" * 19980, "b" * 19980] * 20 + ["a"] * 800, [30000, 1, 1000])

for i in 2 .. 3
  len = 10
  strlen = 80000
  ary = Array.new(len)
  for j in 0 ... len
    s = ""
    for k in 0 ... strlen
      s += (rand(26) + 10).to_s(36)
    end
    ary[j] = s
  end
  x = rand(len * (len - 1))
  d = rand(len * (len - 1))
  generate("test-tough-#{i}.txt", ary, [3000000, x, d])
end
