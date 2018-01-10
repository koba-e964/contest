n = gets.to_i
a = gets.split.map(&:to_i)

for i in 0 ... n
  for j in i ... n
    # increasing in [0, i]?
    ok = true
    for k in 0 ... i
      if a[k] >= a[k + 1]
        ok = false
        break
      end
    end
    if not ok
      next
    end
    for k in j ... n - 1
      if a[k] <= a[k + 1]
        ok = false
        break
      end
    end
    if not ok
      next
    end
    if a[i .. j].uniq.size != 1
      next
    end
    puts 'YES'
    exit
  end
end
puts 'NO'
