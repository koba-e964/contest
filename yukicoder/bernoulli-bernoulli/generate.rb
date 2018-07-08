def generate(filename, n, k)
  fp = open("test_in/" + filename, "w")
  fp.puts(sprintf("%d %d", n, k))
  fp.close
end

srand(0xe869120)
for i in 1 .. 15
  generate("test-#{i}.txt", rand(10 ** 16) + 1, 9001 + rand(1000))
end
