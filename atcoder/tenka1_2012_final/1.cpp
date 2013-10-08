#include <stdio.h>

typedef long long i64;
const int FIBSIZE=60;
i64 fib[FIBSIZE];
const i64 MSIZE=10000000;
int memo[MSIZE]={0};
/**
@retval <= macount
*/
int minc(i64 num,int maxcount=0x40000000)
{
	if(num<0)return -1;
	if(num==0)return 0;
	if(num<MSIZE && memo[num]>0)
	{
		if(maxcount<memo[num])
			return maxcount;
		return memo[num];
	}
	
	//get max t,F_t<=num
	int t=0;
	while(t<51)
	{
		if(num<fib[t])
		{
			t--;
			break;
		}
		t++;
	}
#if 0
	//forall i ,F_i<= num
	for(int i=t;i>=t-7 && i>=1 && maxcount>=1;i--)//F_0 is not worth trying
	{
		int div=minc(num-fib[i],maxcount-1);
		if(maxcount>div+1)maxcount=div+1;
	}
#else
	if(t==51)t=50;
	int div=minc(num-fib[t],maxcount-1);
	maxcount=div+1;
#endif
	if(num<MSIZE)
	{
		memo[num]=maxcount;
	}
	return maxcount;
}

int minc2(i64 num)
{//greedy
	int cnt=0;
	int t=59;
	while(num>0)
	{
		while(t>=0)
		{
			if(num>=fib[t])
			{
				break;
			}
			t--;
		}
		num-=fib[t];
		if(0)
			printf("%d:%lld->%lld\n",t,num+fib[t],num);
		cnt++;
	}
	return cnt;
}
int main(void)
{
	i64 n;
	scanf("%lld",&n);
	i64 t0=0,t1=1;
	for(int i=0;i<60;i++)
	{
		fib[i]=t0;
		i64 tmp=t0+t1;
		t0=t1;
		t1=tmp;
	}
	int val=minc2(n);
	printf("%d\n",val);
	return 0;
}
