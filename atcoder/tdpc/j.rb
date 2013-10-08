def a(v)
	n=v.size
	c=0
	while(v.index(1)!=nil)
		v[rand(n)]=0
		c+=1
	end
	c
end

def prob(v)
	s=0
	for i in 0...10000
		s+=a(v.clone)
	end
	return s/10000.0
end
