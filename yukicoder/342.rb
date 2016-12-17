# coding: utf-8
s = gets.chomp
wara = 'ｗ'
sp = s.split(/(ｗ+)/)
res = Array.new(100).map{|_| []}
for i in 0 ... (sp.size / 2)
  if sp[2 * i] != ""
    res[sp[2 * i + 1].size] << sp[2 * i]
  end
end

(0 ... 100).reverse_each {|i|
  if res[i].size > 0
    puts res[i]
    exit 0
  end
}
