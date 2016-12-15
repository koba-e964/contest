n=gets.to_i
$d=gets.to_i
$s=gets.chomp

$memo = Array.new($s.size, false)
$memo_mul = Array.new($s.size, false)

def parse(pos)
  if pos >= $s.size
    return nil
  end
  if $memo[pos] != false
    return $memo[pos]
  end
  res = nil
  nxt = nil
  res, nxt = parse_mul(pos)
  if res.nil?
    return $memo[pos]=nil
  end
  res2 = nil
  nxt2 = nil
  if $s[nxt] == '+'
    res2, nxt2 = parse(nxt + 1)
    if res2.nil?
      return $memo[pos]=nil
    end
    return $memo[pos] = [add(res, res2), nxt2]
  end
  return $memo[pos] = [res, nxt]
end
def parse_mul(pos)
  if pos >= $s.size
    return nil
  end
  if $memo_mul[pos] != false
    return $memo_mul[pos]
  end
  res = nil
  nxt = nil
  res, nxt = parse_simple(pos)
  if res.nil?
    return $memo_mul[pos]=nil
  end
  res2 = nil
  nxt2 = nil
  if $s[nxt] == '*'
    res2, nxt2 = parse_mul(nxt + 1)
    if res2.nil?
      return $memo_mul[pos]=nil
    end
    return $memo_mul[pos]=[mul(res, res2), nxt2]
  else
    return $memo_mul[pos]=[res, nxt]
  end
end

def parse_simple(pos)
  res, nxt = parse_d(pos)
  if res
    return [res, nxt]
  end
  res, nxt = parse_obj(pos)
  if res
    return [res, nxt]
  end
  nil
end

def parse_d(pos)
  if $s[pos] != 'd' || $s[pos + 1] != '{'
    return nil
  end
  res, nxt = parse(pos + 2)
  if res.nil? || $s[nxt] != '}'
    return nil
  end
  return [diff(res), nxt + 1]
end

def parse_obj(pos)
  if pos >= $s.size
    return nil
  end
  poly = Array.new($d + 1, 0)
  if '0' <= $s[pos] && $s[pos] <= '9'
    i = $s[pos].ord - 48
    poly[0] = i
    return [poly, pos + 1]
  end
  if $s[pos] == 'x'
    poly[1] = 1
    return [poly, pos + 1]
  end
  nil
end

def diff(poly)
  d = poly.size - 1
  res = Array.new(d + 1, 0)
  for i in 0 .. d - 1
    res[i] = (i + 1) * poly[i + 1]
  end
  res
end
def mul(poly1, poly2)
  d = poly1.size - 1
  res = Array.new(d + 1, 0)
  for i in 0 .. d
    for j in 0 .. d - i
      res[i + j] += poly1[i] * poly2[j]
    end
  end
  res
end

def add(poly1, poly2)
  d = poly1.size - 1
  res = Array.new(d + 1, 0)
  for i in 0 .. d
    res[i] += poly1[i] + poly2[i]
  end
  res
end

for i in 0 ... $s.size
  parse($s.size - i - 1)
end
result, pos = parse(0)
for i in 0 .. $d
  print(result[i])
  print(if i == $d then "\n" else " " end)
end
