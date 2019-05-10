#include <stdio.h>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;

const int N=51;
int h,w;
char s[N][N];

int main(void){
  scanf("%d%d",&h,&w);
  REP(i,0,h)scanf("%60s",s[i]);
  int dx[4]={1,0,-1,0},dy[4]={0,1,0,-1};
  REP(i,0,h){
    REP(j,0,w){
      if(s[i][j]!='#')continue;
      int ex=0;
      REP(d,0,4){
        int ni=i+dx[d],nj=j+dy[d];
        if(ni<0||ni>=h||nj<0||nj>=w)continue;
        if(s[ni][nj]=='#')ex++;
      }
      if(!ex){
        puts("No");
        return 0;
      }
    }
  }
  puts("Yes");
}
