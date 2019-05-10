#include <stdio.h>
#include <map>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long ll;

const int N=100100;
int n;
ll m,a[N],c[N];

int main(void){
  scanf("%d%lld",&n,&m);
  REP(i,0,n){
    scanf("%lld",a+i);
    c[i+1]=c[i]+a[i];
  }
  map<ll,ll> cnt;
  REP(i,0,n+1){
    cnt[c[i]%m]++;
  }
  ll ans=0;
  for(auto e:cnt){
    ll x=e.second;
    ans+=x*(x-1)/2;
  }
  printf("%lld\n",ans);
}
