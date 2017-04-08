n = gets.to_i
if n <= 14
  prod = 1
  for i in 1 .. n
    prod *= i
  end
  puts prod
  exit 0
end

if n >= 100
  puts "0" * 12
  exit 0
end

def fact(n)
  prod = 1
  for i in 1 .. n
    prod = prod * i % 10 ** 12
  end
  prod
end

printf "%012d\n", fact(n)
