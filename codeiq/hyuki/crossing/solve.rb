def reading()
	filename='crossing.txt'
	fp=open(filename,'r')
	ary=[]
	for line in fp
		ary<<line.to_i
	end
	return ary
end
include Math
def bsearch(ary,v) # ary needs to be sorted
	i=0 #x<i ===> ary[x]<=v
	j=ary.size-1 # x>j =====> ary[x]>v
	while i<=j
		m=(i+j)/2
		if (ary[m]<=v)
			i=m+1
		else
			j=m-1
		end
	end
	return i
end
def solve1(ary)
	prep=[]
	n=ary.size
	ans=0
	for i in 0...n
		if i%10000==0
			puts i.to_s+'/'+n.to_s
		end
		v=ary[i]
		k=bsearch(prep,v)
		ans+=i-k
		prep.insert(k,v)
	end
	return ans
end
def solve2(ary)
	n=ary.size
	m=1<<log2(n+1).ceil # guarantees that cnt[1..n] is valid
	ans=0
	cnt=Array.new(m,0) # cnt[...01110000]:ary[0]~ary[i]‚ÅA[....01110000, ....10000000)‚É‘¶Ý‚·‚é‚à‚Ì‚ÌŒÂ”
	for i in 0...n
		if i%10000==0
			puts i.to_s+'/'+n.to_s
		end
		v=ary[i] # v is in [1,n]
		w=v
		c=0
		while(w<m)
			least=w&(-w)
			c+=cnt[w]
			w+=least
		end
		ans+=c
		w=v
		while(w>0)
			least=w&(-w)
			cnt[w]+=1
			w-=least
		end
	end
	return ans
end

def test()
	ary=reading()
	#solution 1
	cl=ary.clone
	st=Time.now
	res=solve1(cl)
	en=Time.now
	puts "solution 2:"
	puts "result="+res.to_s
	puts "time:"+(en-st).to_s+" sec"

	#solution 2
	cl=ary.clone
	st=Time.now
	res=solve2(cl)
	en=Time.now
	puts "solution 2:"
	puts "result="+res.to_s
	puts "time:"+(en-st).to_s+" sec"
end
