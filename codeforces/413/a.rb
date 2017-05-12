n, t, k, d = gets.split.map(&:to_i)
fst = (n + k - 1) / k * t
snd = fst + 1
for i in 0 .. fst
  tmp = i / t * k
  if i >= d
    tmp += (i - d) / t * k
  end
  if tmp >= n
    snd = i
    break
  end
end
puts (fst > snd ? "YES" : "NO")
