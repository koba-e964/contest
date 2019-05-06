while 1
  s=gets.chomp.split(/ /)
  if s==['END','OF','INPUT'];exit 0;end
  for v in s
    print v.size
  end
  puts
end
