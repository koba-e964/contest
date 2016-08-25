n = gets.to_i
if n == 1 || n == 2
  puts -1
  exit
end

if (n & (-n)) == n
  puts [3 * n / 4, 5 * n / 4]*' '
else
  d = n & (-n)
  q = n / d
  puts [(q * q - 1) / 2 * d, (q * q + 1) / 2 * d]*' '
end

