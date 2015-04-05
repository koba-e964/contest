#include <stdio.h>

int d[101];

int main(void)
{
	int n;
	scanf("%d",&n);
	int sum=0;	
	for(int i=0;i<n;i++)
	{
		scanf("%d",d+i);
		sum+=d[i];
	}
	int s,t;
	scanf("%d%d",&s,&t);
	int p1=0;
	if(s>t)
	{
		int tmp=t;
		t=s;
		s=tmp;
	}
	for(int i=s;i<t;i++)
	{
		p1+=d[i-1];
	}
	printf("%d\n",p1>sum-p1?sum-p1:p1);
	return 0;
}
