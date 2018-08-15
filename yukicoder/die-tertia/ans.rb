y, m, d = gets.split('/').map(&:to_i)
date_limit = [nil,
              31,28,31,
              30,31,30,
              31,31,30,
              31,30,31]

if (y % 4 == 0) ^ (y % 100 == 0) ^ (y % 400 == 0)
  date_limit[2] += 1
end

d += 2
if d > date_limit[m]
  d -= date_limit[m]
  m += 1
end

if m >= 13
  m -= 12
  y += 1
end

puts sprintf("%04d/%02d/%02d", y, m, d)
