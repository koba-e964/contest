s=gets.chop
puts ('a'..'z').find{|v|s.count(v)%2>0}.nil? ? :Yes: :No
