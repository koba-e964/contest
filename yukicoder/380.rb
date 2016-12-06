def search(s, t)
  if s.size < t.size
    return false
  end
  for i in 0 .. [3, s.size - t.size].min
    ch = s[-i].downcase
    if i >= 1 && (('0' <= ch && ch <= '9') || ('a' <= ch && ch <= 'z'))
      return false
    end
    if s[-t.size-i..-1-i] == t
      return true
    end
  end
  false
end


while true
  s = gets
  if s.nil?
    exit 0
  end
  s = s.chomp
  ok = false
  if s[0..4] == "digi "
    ok = search(s.downcase[5..-1], "nyo")
  end
  if s[0..5] == "petit "
    ok = search(s.downcase[6..-1], "nyu")
  end
  if s[0..4] == "gema "
    ok = search(s.downcase[5..-1], "gema")
  end
  if s[0..4] == "piyo "
    ok = search(s[5..-1].downcase, "pyo")
  end
  if s[0..4] == "rabi "
    t = s[5..-1].downcase
    for k in t.chars
      if ('0' <= k && k <= '9') || ('a' <= k && k <= 'z')
        ok = true
      end
    end
  end
  puts (ok ? "CORRECT (maybe)" : "WRONG!")
end
