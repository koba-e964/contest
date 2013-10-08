#include <stdio.h>

int solve(int n,int m,int l,int p,int q,int r);
int main()
{
	int n,m,l,p,q,r;
	scanf("%d%d%d%d%d%d",&n,&m,&l,&p,&q,&r);
	int max=0;
	int res=0;
	res=solve(n,m,l,p,q,r);if(max<res)max=res;
	res=solve(n,m,l,p,r,q);if(max<res)max=res;
	res=solve(n,m,l,q,p,r);if(max<res)max=res;
	res=solve(n,m,l,q,r,p);if(max<res)max=res;
	res=solve(n,m,l,r,q,p);if(max<res)max=res;
	res=solve(n,m,l,r,p,q);if(max<res)max=res;
	printf("%d\n",max);
	return 0;
}

int solve(int n,int m,int l,int p,int q,int r)
{
	return (n/p)*(m/q)*(l/r);
}