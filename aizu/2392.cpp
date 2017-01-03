#include <stdio.h>
#define TEST 0
int gcd(int a,int b)
{
	while(b!= 0)
	{
		int r=a%b;
		a=b;
		b=r;
	}
	return a;
}

int lcm(int a,int b)
{
	return a/gcd(a,b) *b;
}
int inv(int x,int mod)
{
	int
	a00=1,a01=0,
	a10=0,a11=1;
	int b0=x,b1=mod ; // vector b = A  t(x mod)
	while(b1 != 0)
	{
		int r= b0 % b1,q= b0 /b1;
		b0=b1;
		b1=r;
		//(0 1;1 -q)->A
		int c00=a10,c01=a11,
			c10=a00 -q*a10,c11=a01-q*a11;
		a00=c00;
		a01=c01;
		a10=c10;
		a11=c11;
	}
	if(b0 != 1)
		throw -1;//gcd != 1
	return a00 % mod;
}

int idiv(int a,int b,int mod)
{
	int g=gcd(b,mod);
	if(a % g != 0)
		throw 2;
	return (a/g) * inv(b/g,mod/g) % mod;
}

int totientf(int m)
{
	int divisor=2;//prime
	int res=1;
	int pow=0;
	if(m <= 0)
		return 1;
	while(1)
	{
		if(m % divisor == 0)
		{
			if(pow == 0)
				pow =divisor -1;
			else
				pow *= divisor;
			m/=divisor;
		}
		else // pow || m
		{
			if(pow != 0)
			{
				printf("pow=%d\n",pow);
				res=lcm(res,pow);
				pow=0;
			}
			if(m == 1)//end
				break;
			divisor++;
		}
	}
	return res;
}

int res(int n,int m,int l)
{
	switch(n)
	{
	case 0:
		return 1;
	case 1:
		return l%m;
	case 2:
		return ((l%m)*2+m-1)%m;
	default:
		{}
	}
	int exp=idiv((res(n-1,totientf(m)*(l-1),l)-1),(l-1),totientf(m));
	long cur=l;
	long mult=1;
	while(exp > 0)
	{
		if(exp & 1)
		{
			mult *= cur;
			mult %= m;
		}
		exp >>=1;
		cur *= cur;
		cur %= m;
	}
	return (int)((mult*2-1)%m);
}
#if TEST
int main(void)
{
	printf("tot(%d)=%d",30,totientf(30));
}
#else
int main(void)
{
	int n,m,l;
	int c=0;
	while(1)
	{
		scanf("%d%d%d",&n,&m,&l);
		if(n== 0 && m== 0  && l== 0)
			break;
		printf("Case %d: %d\n",c+1,res(n,m,l));
		c++;
	}
}
#endif