#include <stdio.h>
#include <math.h>
int main(void)
{
	int n;
	int sum=0;
	scanf("%d",&n);
	for(int i=0;i<n;i++)
	{
		int a,b;
		scanf("%d%d",&a,&b);
		sum+=a*b;
	}
	sum=(int)floor(sum*1.05);
	printf("%d\n",sum);
	return 0;
}