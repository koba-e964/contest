n = gets.to_i
a = gets.split.map(&:to_i)

cur_ma = a[0]
cur_mi = a[0]
for i in 1 ... n
  op = a[i]
  ma = -(10 ** 50)
  mi = 10 ** 50
  if op != 0
    for c in [cur_ma, cur_mi]
      o = op
      t = 0
      if c < 0
        t = 1 - t
        c = -c
      end
      if o < 0
        t = 1 - t
        o = -o
      end
      ma = [ma, (-1) ** t * (c / o)].max
      mi = [mi, (-1) ** t * (c / o)].min
    end
  end
  ma = [ma, cur_ma * op].max
  ma = [ma, cur_ma + op].max
  ma = [ma, cur_ma - op].max
  ma = [ma, cur_mi * op].max
  ma = [ma, cur_mi + op].max
  ma = [ma, cur_mi - op].max
  mi = [mi, cur_ma * op].min
  mi = [mi, cur_ma + op].min
  mi = [mi, cur_ma - op].min
  mi = [mi, cur_mi * op].min
  mi = [mi, cur_mi + op].min
  mi = [mi, cur_mi - op].min
  cur_ma = ma
  cur_mi = mi
end
puts cur_ma
