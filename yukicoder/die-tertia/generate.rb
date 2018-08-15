def generate(filename, y, m, d)
  fp = open("test_in/" + filename, "w")
  fp.puts(sprintf("%04d/%02d/%02d", y, m, d))
  fp.close
end

date_limit = [nil,
              31,28,31,
              30,31,30,
              31,31,30,
              31,30,31]


srand(0xe869120)
for i in 1 .. 12
  generate("test-#{i}.txt", 2000 + rand(400), i, date_limit[i])
end

# extreme case
generate("test-13.txt", 2400, 12, 31)

leap_check = [2000, 2016, 2100, 2200, 2400]
for i in 0 ... leap_check.size
  generate("test-#{14 + i}.txt", leap_check[i], 2, 28)
end
