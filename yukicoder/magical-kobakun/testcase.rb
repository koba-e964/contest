n_small_testcase = 20
n_testcase = 60
n_evil_testcase = 10
small_max = 300
max = 10 ** 9
for i in 0 ... n_small_testcase
  fp = open("test_in/testcase-small-#{i}.txt", 'w')
  x = rand(small_max) + 1
  y = rand(small_max) + 1
  fp.puts("#{x} #{y}")
  fp.close
end

for i in 0 ... n_testcase
  fp = open("test_in/testcase-#{i}.txt", 'w')
  x = rand(max) + 1
  y = rand(max) + 1
  fp.puts("#{x} #{y}")
  fp.close
end

for i in 0 ... n_evil_testcase
  fp = open("test_in/testcase-evil-#{i}.txt", 'w')
  x = rand(max) + 1
  y = rand(3) + 1
  fp.puts("#{x} #{y}")
  fp.close
end

