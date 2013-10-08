#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int toCo(int num,int *x,int *y)
{
	int k=ceil(sqrt(2*num+0.25)-0.5);
	*x=k;
	*y=num-k*(k-1)/2;
}
int toVal(int x,int y)
{
	return x*(x-1)/2+y;
}

int main(void)
{
	int i,j;
	scanf("%d%d",&i,&j);
	int ix,iy,jx,jy;
	toCo(i,&ix,&iy);
	toCo(j,&jx,&jy);
	printf("%d\n",toVal(ix+jx+1,iy+jy));
	return 0;
}
