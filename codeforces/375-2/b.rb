gets
s=gets.chomp
q=s.scan(/\(\w*\)/).map{|v|v[1...v.size-1]}*'_'
r=s.split(/\(\w*\)/)*'_'
puts "#{(r.split('_')+[""]).map{|v|v.size}.max} #{q.split('_').select{|v|v!=""}.size}"
