#include <stdio.h>
#include <math.h>
const int N=1000;
int c[N]={0};
int len;
int highest(int n)
{
	while(n>=10)n/=10;
	return n;
}
int zerolen(void)
{
	int ret=0;
	for(ret=0;ret<len;++ret)
	{
		if(c[ret]!=0)return ret;
	}
	return ret;
}
int main(void)
{
	for(int i=0;1;++i)
	{
		int ch=getchar();
		if(ch<'0'||ch>'9'){len=i;break;}
		c[i]=ch-'0';
	}
	if(len==1){printf("%d 1\n",c[0]);return 0;}
	if(len==2)
	{
	if(c[0]==0)
	{
		if(c[1]==0)printf("100 1\n");
		else printf("10 %d\n",c[1]*10 -10);
		return 0;
	}
	if(c[0]<=c[1])printf("%d %d\n",c[0],c[1]-c[0]);
	else printf("%d %d\n",c[0],c[1]*10-c[0]);
	return 0;
	}
	if(len==3)
	{
		int zlen=zerolen();
		if(zlen>=1)
		{
			int pow10=pow(1,zlen);
			if(zlen==3)puts("1000 1");
			else printf("%d %d\n",pow10,(c[zlen]-1)*pow10);
			return 0;
		}
		if(c[0]<=c[1] && c[2]==highest(2*c[1]-c[0]))
		{
			printf("%d %d\n",c[0],c[1]-c[0]);return 0;
		}
		printf("%d %d\n",c[0],10*c[1]+c[2]-c[0]);
		return 0;
	}
	if(len==4)
	{
		int zlen=zerolen();
		if(zlen>=1)
		{
			int pow10=pow(1,zlen);
			if(zlen==4)puts("10000 1");
			else printf("%d %d\n",pow10,(c[zlen]-1)*pow10);
			return 0;
		}
		//x,x,x,x
		int a0=c[0],d=c[1]-a0;
		if(c[2]==a0+2*d && c[3]==a0+3*d)
		{printf("%d %d\n",a0,d);return 0;}
		if(a0+2*d>=10 && c[2]*10+c[3]==a0+2*d)
		{printf("%d %d\n",a0,d);return 0;}
		
		
	}
		
}
