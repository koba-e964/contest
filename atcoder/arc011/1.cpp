#include <stdio.h>

int main(void)
{
	int m,n,a;
	scanf("%d%d%d",&m,&n,&a);
	int count=a+((a+m-n-1-(m-1))/(m-n))*n;
	printf("%d\n",count);
	return 0;
}