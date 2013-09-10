#include <stdio.h>
#include <stdlib.h>
typedef long long int i64;
const int MOD=1777777777;

int mo(int k)
{
	i64 cur=1;
	i64 sum=0;
	for(int i=k;i>=2;i--)
	{
		sum+=(i%2)?-cur:(cur);
		sum%=MOD;
		cur*=i;
		cur%=MOD;
	}
	return (int)(sum%MOD);
}

i64 comb(i64 n,int k)
{
	n%=MOD;
	i64 prod=1;
	for(int i=0;i<k;i++)
	{
		prod*=n-i;
		prod/=i+1;
	}
	return prod%MOD;

}


int solve(i64 n,int k)
{
	i64 cm=comb(n,k)%MOD;
	return (int)((MOD+cm*mo(k))%MOD);
}




int main(void)
{
	i64 n;
	int k;
	scanf("%lld%d",&n,&k);
	printf("%d\n",(int)solve(n,k));
	return 0;
}
