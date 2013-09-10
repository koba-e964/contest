#include <stdio.h>

#define N 19
int board[19][19];
int cont(int c,int l,int x,int y,int dx,int dy)
{
	for(int i=0;i<l;i++)
	{
		if(x>=N || y>=N ||y<0|| board[x][y]!=c)return 0;
		x+=dx;
		y+=dy;
	}
	return 1;
}

int search(int c,int l)
{
	int cnt=0;
	for(int i=0;i<N;i++)
	{
		for(int j=0;j<N;j++)
		{
			if(board[i][j]==c)
			{
				if(
cont(c,l,i,j,1,0)||
cont(c,l,i,j,0,1)||
cont(c,l,i,j,1,1)||
cont(c,l,i,j,1,-1))cnt++;
			}
		}
	}
	return cnt;
}

int main(void)
{
	int nb=0,nw=0;
	for(int i=0;i<N;i++)
	{
		for(int j=0;j<N;)
		{
			int ch=getchar();
			switch(ch)
			{
			case 'o':board[i][j]=3;j++;nb++;break;
			case 'x':board[i][j]=2;j++;nw++;break;
			case '.':board[i][j]=1;j++;break;
			default:{}
			}
		}
	}
	if(nb-nw<=-1 || nb-nw>=2)
	{
		puts("NO");
		return 0;
	}
	if(nb==nw)
	{
		puts(search(3,5)>=1||search(2,10)>=1||search(2,5)>=2?"NO":"YES");
		return 0;
	}
	puts(search(2,5)||search(3,10)||search(3,5)>=2?"NO":"YES");
	return 0;
}
