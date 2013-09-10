#include <stdio.h>

int main(void)
{
	int n,a,b,l;
	scanf("%d%d%d%d",&n,&a,&b,&l);
	double cur=l;
	double vd=(double)a/(double)b;
	for(int i=0;i<n;i++)
	{
		cur/=vd;
	}
	printf("%f\n",cur);
	return 0;
}
