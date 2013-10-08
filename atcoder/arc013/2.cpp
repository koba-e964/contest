#include <stdio.h>
#include <stdlib.h>

#define MAX 100
int n[MAX],m[MAX],l[MAX];
int comp(const void* a,const void *b)
{
	return *(int*)a - *(int*)b;
}
int main()
{
	int c;
	scanf("%d",&c);
	int nm=1,mm=1,lm=1;
	for(int i=0;i<c;i++)
	{
		int tmp[3];
		scanf("%d%d%d",tmp,tmp+1,tmp+2);
		qsort(tmp,3,sizeof(int),comp);
		if(nm<tmp[0])nm=tmp[0];
		if(mm<tmp[1])mm=tmp[1];
		if(lm<tmp[2])lm=tmp[2];
	}
	printf("%d\n",nm*mm*lm);
	return 0;
}
