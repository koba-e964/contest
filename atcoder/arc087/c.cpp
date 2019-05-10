#include <stdio.h>
#include <map>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;

const int N=100100;
int n,a[N];
map<int,int>f;
int main(void){
  scanf("%d",&n);
  REP(i,0,n)scanf("%d",a+i);
  REP(i,0,n){
    f[a[i]]++;
  }
  int ans=0;
  for(auto e:f){
    ans+=e.second-(e.second>=e.first?e.first:0);
  }
  printf("%d\n",ans);
}
