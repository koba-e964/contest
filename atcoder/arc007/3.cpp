#include <stdio.h>
#include <stdlib.h>
int rot[10][10];
int row[10];

int main(void)
{
	char pat[10];
	int len;
	srand(0x875);
	for(int i=0;1;i++)
	{
		int ch=getchar();
		if(ch!='o' && ch != 'x')
		{
			len=i;
			break;
		}
		pat[i]=ch;
	}
	for(int i=0;i<len;i++)
	{
		for(int j=0;j<len;j++)
		{
			rot[i][j]=(pat[(i+j)%len]=='o')?1:0;
			row[i]+=rot[i][j];
		}
	}
	int mincnt=0x500;

	for(int bit=0;bit<(1<<len);bit++)
	{
		int cnt=0;
		int ok[10]={0};
		int rem=len;
		for(int i=0;i<len;i++)
		{
			if(bit&(1<<i))
			{
				for(int j=0;j<len;j++)
				{
					if(rot[i][j]){rem-=1-ok[j];
						ok[j]=1;}
				}
				cnt++;
			}
		}
		if(rem==0 && mincnt>cnt)mincnt=cnt;
	}
#if 0
	while(1)
	{
		int maxi=0;//row
		
		for(int i=1;i<len;i++)
		{
			if(row[i]>row[maxi])maxi=i;
			if(row[i]==row[maxi] && (rand()&1))
			{
				maxi=i;
			}
		}
		if(row[maxi]==0)break;//no blanks
		for(int j=0;j<len;j++)
		{
			if(rot[maxi][j])
			{
				for(int i=0;i<len;i++)
				{
					if(rot[i][j])
					{
						row[i]--;
						rot[i][j]=0;
					}
				}
			}
		}
		cnt++;
	}
#endif
	printf("%d\n",mincnt);
	return 0;
}
