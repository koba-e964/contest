#include <stdio.h>
#define TEST 0
#define INF 0x7fffffff

int main()
{
	int n;
	scanf("%d",&n);
	int gru=0;
	for(int i=0;i<n;i++)
	{
		int x0=INF,x1=0,y0=INF,y1=0,z0=INF,z1=0;
		int x,y,z,m;
		scanf("%d%d%d%d",&x,&y,&z,&m);
		for(int j=0;j<m;j++)
		{
			int a,b,c;
			scanf("%d%d%d",&a,&b,&c);
#define mac(p,q,r) if(p>r)p=r;if(q<r)q=r;
		mac(x0,x1,a)
		mac(y0,y1,b)
		mac(z0,z1,c)
		}
		if(TEST)
			printf("[%d,%d]*[%d,%d]*[%d,%d]",x0,x1,y0,y1,z0,z1);
		int tmp=0;
		tmp^=x0;
		tmp^=x-x1-1;
		tmp^=y0;
		tmp^=y-y1-1;
		tmp^=z0;
		tmp^=z-z1-1;
		gru^=tmp;
	}
	puts(gru?"WIN":"LOSE");
	return 0;
}
