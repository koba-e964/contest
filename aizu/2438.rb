def parse(yaml, pos, props, cur_lv = -1)
  cur = ""
  while pos < yaml.size
    line = yaml[pos]
    lv = 0
    while line[0] == ' '
      line = line[1..-1]
      lv += 1
    end
    if lv <= cur_lv
      return pos
    end
    dat = line.split(':')
    if dat.size == 1
      child = {}
      # it has children
      pos = parse(yaml, pos + 1, child, lv)
      props[dat[0]] = child
    else
      props[dat[0]] = dat[1][1..-1]
      pos += 1
    end
  end
  return pos
end


query = gets.chomp.split(".")[1..-1]
yaml = []
while s = gets
  yaml << s.chomp
end

props = {}
parse(yaml, 0, props)
for v in query
  if props.is_a?(Hash) && props.member?(v)
    props = props[v]
  else
    puts "no such property"
    exit 0
  end
end

if props.is_a?(String)
  puts "string \"#{props}\""
else
  puts "object"
end
