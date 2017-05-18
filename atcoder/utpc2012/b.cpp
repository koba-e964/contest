#include <stdio.h>

int chs[0x100];


int main(void)
{
	int cnt=0;
	while(1)
	{
		int ch=getchar();
		if(ch==-1 || ch=='\n')break;
		chs[cnt]=ch-'A';
		cnt++;
	}
	int res[8]={0};
	int rc=7;
	for(int i=cnt-1;i>=0&& rc>=0;i--)
	{
		if(res[chs[i]]==0)
		{
			res[chs[i]]=rc;
			rc--;
		}
	}
	for(;rc>=0;)
	{
		for(int i=0;i<8;i++)
		{
			if(res[i]==0)
			{
				res[i]=rc;
				rc--;
			}
		}
	}
	int pc=0;
	while(pc<8)
	{
		for(int i=0;i<8;i++)
		{
			if(res[i]==pc)
			{
				printf("%c",'A'+i);
				pc++;
				break;
			}
		}
	}
	puts("");
	return 0;
}
