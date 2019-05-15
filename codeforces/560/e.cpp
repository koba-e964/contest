#include <stdio.h>
#include <algorithm>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

typedef long long ll;
int n;
ll a[200100],b[200100],c[200100];

const ll mo=998244353;

int main(void){
  scanf("%d",&n);
  REP(i,0,n)scanf("%lld",a+i);
  REP(i,0,n)scanf("%lld",b+i);
  REP(i,0,n)c[i]=ll(i+1)*ll(n-i)*a[i];
  sort(c,c+n);
  sort(b,b+n);
  ll ans=0;
  REP(i,0,n){
    c[i]%=mo;
    ans+=b[n-1-i]*c[i];
    ans%=mo;
  }
  printf("%lld\n",ans);
}
