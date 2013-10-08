#include <stdio.h>

int cnt[10]={0};

int TEST=1;

int main(void)
{
	int nums=0;
	while(nums<=2)
	{
		int ch=getchar();
		if(ch==-1)
		{
			break;
		}
		if(ch=='/')
		{
			nums++;
			continue;
		}
		int num=ch-'0';
		if(num<0 || num>=10)
			continue;
		if(nums==0)
			cnt[num]++;
		else
			cnt[num]--;
	}
	int succ=1;
	int i;
	for(i=0;i<10&&succ;i++)
	{
		succ=cnt[i]==0;
	}
	if(TEST)
	{
		printf("(");
		for(i=0;i<10;i++)
		{
			printf("%d ",cnt[i]);
		}
		puts(")");
	}
	puts(succ?"yes":"no");
	return 0;
}
