#include <stdio.h>
#include <stdlib.h>
#define MAX 20
#define V 8001
int x[MAX],y[MAX],z[MAX];
char graph[V][V];
int main()
{
	for(int i=0;i<V;i++)
		for(int j=0;j<V;j++)
			graph[i][j]=0;
	int n;
	scanf("%d",&n);
	if(n>=2)abort();
	for(int i=0;i<n;i++)
	{
		int x,y,z;
		scanf("%d%d%d",&x,&y,&z);
		int v=x*y*z;
		for(int j=1;j<x-1;j++)
		{
			int bra=j*y*z;
			graph[x-bra][bra]=1;
			graph[bra][x-bra]=1;
		}
		for(int j=1;j<y-1;j++)
		{
			int bra=j*x*z;
			graph[y-bra][bra]=1;
			graph[bra][y-bra]=1;
		}
		for(int j=1;j<z-1;j++)
		{
			int bra=j*y*x;
			graph[z-bra][bra]=1;
			graph[bra][z-bra]=1;
		}
	}
	int count=0;
	for(int i=0;i<V;i++)
		for(int j=0;j<V;j++)
			count+=graph[i][j]?(i==j?1:2):0;
	printf("%d\n",count);
	return 0;
}
