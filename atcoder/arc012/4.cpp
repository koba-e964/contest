#include <stdio.h>
typedef long long int i64;
#define NMAX 100000

int invmod(int a,int p)
{
	int pow=p-2;
	i64 sum=1;
	i64 cur=a;
	while(pow>0)
	{
		if(pow&1)
		{
			sum*=cur;
			sum%=p;
		}
		cur*=cur;
		cur%=p;
	}
	return (int)sum;
}

int comb(int n,int k,int mod)
{
	i64 mul=1;
	for(int i=0;i<k;i++)
	{
		mul*=n-i;
		mul/=i+1;
	}
	return (int)(mul%(i64)mod);
}
int x[NMAX],y[NMAX];
int comb_p(int n,int k,int mod)
{
	i64 mul=1;
	for(int i=0;i<k;i++)
	{
		mul*=n-i;
		mul%=mod;
		mul*=invmod(i+1,mod);
		mul%=mod;
	}
	return (int)(mul%(i64)mod);

}

int get_pat(int x,int y,int t,int mod)
{
	if((x+y+t)%2==1)return 0;
	if(x+y>t)return 0;
	int sum=0;
	for(int i=0;i<(x+y-t)/2;i++)
	{
		i64 mul=comb(
	}
}

int main(void)
{
	int n,t,mod;
	scanf("%d%d%d",&n,&t,&mod);
	for(int i=0;i<n;i++)
	{
		scanf("%d%d",x+i,y+i);
		if(x[i]<0)x[i]=-x[i];
		if(y[i]<0)y[i]=-y[i];
	}
	
	
}
