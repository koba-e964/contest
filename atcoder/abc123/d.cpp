#include <algorithm>
#include <stdio.h>
#include <vector>

#define REP(i,s,n) for(int i=(s);i<(n);++i)
typedef long long ll;
using namespace std;
const int N=1010;

int x,y,z,k;
ll a[N],b[N],c[N];

vector<ll> ab;
vector<ll> abc;

int main(){
  scanf("%d%d%d%d",&x,&y,&z,&k);
  REP(i,0,x)scanf("%lld",a+i);
  REP(i,0,y)scanf("%lld",b+i);
  REP(i,0,z)scanf("%lld",c+i);
  REP(i,0,x)REP(j,0,y)ab.push_back(a[i]+b[j]);
  sort(ab.rbegin(),ab.rend());
  REP(i,0,z){
    REP(j,0,min(k,int(ab.size()))){
      abc.push_back(c[i]+ab[j]);
    }
  }
  sort(abc.rbegin(),abc.rend());
  REP(r,0,k){
    printf("%lld\n",abc[r]);
  }
}
