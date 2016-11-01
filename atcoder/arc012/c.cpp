#include <stdio.h>

#define N 19
#define REP(i, s, n) for (int i = (s); i < (n); ++i)
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

bool check(int col) {
  bool white_valid = search(5 - col, 5) == 0;
  bool ok = 0;
  REP(i, 0, N) {
    REP(j, 0, N) {
      if (board[i][j] != col) { continue; }
      board[i][j] = 1;
      ok |= search(col, 5) == 0;
      board[i][j] = col;
    }
  }
  return ok && white_valid;
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
  if (nb + nw == 0) {
    puts("YES");
    return 0;
  }
  if(nb==nw)
    {
      puts(check(2)?"YES":"NO"); // Try removing 'x'
      return 0;
    }
  puts(check(3)?"YES":"NO"); // Try removing 'o'
  return 0;
}
