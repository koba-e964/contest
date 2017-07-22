def one_level(dig, pat)
  if dig == 9
    return pat[0] + pat[2]
  end
  if dig == 4
    return pat[0] + pat[1]
  end
  if dig >= 5
    return pat[1] + pat[0] * (dig - 5)
  end
  return pat[0] * dig
end

def number_to_roman(num)
  ret = 'M' * (num / 1000)
  num -= num / 1000 * 1000
  for v, pat in [[100, 'CDM'], [10, 'XLC'], [1, 'IVX']]
    dig = num / v
    num -= dig * v
    ret += one_level(dig, pat)
  end
  ret
end

def roman_to_number(str)
  val = 0
  dict = {'IV' => -2, 'IX' => -2, 'XL' => -20, 'XC' => -20, 'CD' => -200,
          'CM' => -200}
  for pat, diff in dict
    if str.include?(pat)
      val += diff
    end
  end
  dict = {'I' => 1, 'V' => 5, 'X' => 10, 'L' => 50, 'C' => 100,
          'D' => 500, 'M' => 1000}
  for letter, diff in dict
    val += diff * str.count(letter)
  end
  val
end

_ = gets.to_i
r = gets.chomp.split
sum = 0
for v in r
  sum += roman_to_number(v)
end
if sum >= 4000
  puts "ERROR"
else
  puts number_to_roman(sum)
end
