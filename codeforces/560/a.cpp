#include <stdio.h>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

int n,x,y;
char a[200100];

int main(void){
  scanf("%d%d%d%200099s",&n,&x,&y,a);
  int ans=0;
  REP(i,0,x){
    int t=i==y?'1':'0';
    ans+=t!=a[n-1-i];
  }
  printf("%d\n",ans);
}
