#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define N 100
#define TEST 0
int comp_int(const void *a,const void* b)
{
	return *(int*)b-*(int*)a;
}
int main(void)
{
	int n;
	int a[10];
	char c[N][7];
	scanf("%d",&n);
	for(int i=0;i<10;i++)
	{
		scanf("%d",a+i);
	}
	for(int i=0;i<n;i++)
	{
		char buf[8];
		scanf("%*[\n]%7[X-]",buf);
		memcpy(c[i],(const void*)buf,7);
	}
	int max[7]={0};
	for(int i=0;i<7;i++)
	{
		int cont=0;
		for(int j=0;j<n;j++)
		{
			if(c[j][i]=='X')
			{
				cont++;
				continue;
			}
			if(max[i]<cont)
				max[i]=cont;
			cont=0;
		}
		if(max[i]<cont)
			max[i]=cont;
	}
	qsort(max,7,4,comp_int);
	qsort(a,10,4,comp_int);

#if TEST
	printf("a:");
	for(int i=0;i<10;i++)
	{
		printf("%d ",a[i]);
	}
	printf("max:");
	for(int i=0;i<7;i++)
	{
		printf("%d",max[i]);
	}
#endif
	//allocate
	bool ok=true;
	for(int i=0;i<7 && ok;i++)
	{
		if(a[i]<max[i])
			ok=false;
	}
	printf(ok?"YES\n":"NO\n");
	return 0;
}
