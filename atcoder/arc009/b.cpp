#include <stdio.h>
#include <algorithm>

const int MAXN=1000;
int c[10];
struct comp
{
	static int pow10[10];
	comp(){
		int k=1;
		for(int i=0;i<10;i++)
		{
			pow10[i]=k;
			k*=10;
		}
	}
	comp(const comp&)
	{}
	bool operator()(int a,int b)
	{
		for(int i=9;i>=1;i--)
		{
			int diga=c[(a/pow10[i-1])%10];
			int digb=c[(b/pow10[i-1])%10];
			if(diga!= digb)
				return diga<digb;
		}
		return false;
	}
};
int comp::pow10[10];
int main(void)
{
	for(int i=0;i<10;i++)
	{
		int val;
		scanf("%d",&val);
		if(val<0 && val>=10)continue;
		c[val]=i;
	}
	int n;
	scanf("%d",&n);
	int a[MAXN];
	comp cm;
	for(int i=0;i<n;i++)
	{
		scanf("%d",a+i);
	}
	std::sort(a,a+n,cm);
	for(int i=0;i<n;i++)
	{
		printf("%d\n",a[i]);
	}
	return 0;
}
