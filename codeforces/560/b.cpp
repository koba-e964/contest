#include <stdio.h>
#include <algorithm>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

int n,a[200100];
int f[200100];

int main(void){
  scanf("%d",&n);
  REP(i,0,n)scanf("%d",a+i),f[a[i]]++;
  sort(a,a+n);
  int ans=0;
  REP(i,0,n){
    if(a[i]>=ans+1)ans++;
  }
  printf("%d\n",ans);
}
