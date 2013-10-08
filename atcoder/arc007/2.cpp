#include <stdio.h>

int main(void)
{
	int ary[101];//ary[0]:cd player
	int n,m;
	scanf("%d%d",&n,&m);
	for(int j=0;j<n+1;j++)
	{
		ary[j]=j;
	}
	for(int i=0;i<m;i++)
	{
		int disk;
		scanf("%d",&disk);
		for(int j=0;j<n+1;j++)
		{//find disk in ary
			if(ary[j]==disk)
			{
				if(j!=0)
				{//swap
					int tmp=ary[j];
					ary[j]=ary[0];
					ary[0]=tmp;
				}
				break;
			}
		}
	}
	for(int i=0;i<n;i++)
	{
		printf("%d\n",ary[i+1]);
	}
	return 0;
}
