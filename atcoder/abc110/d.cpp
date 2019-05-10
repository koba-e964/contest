#include <stdio.h>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long ll;
const ll mod=1e9+7;

int n;
ll m;

ll powmod(ll x,ll e){
  ll c=x,p=1;
  while(e>0){
    if(e%2)p=p*c%mod;
    c=c*c%mod;
    e/=2;
  }
  return p;
}

ll f(int e){
  ll ans=1;
  REP(i,0,e){
    ans=ans*(n+i)%mod;
    ans=ans*powmod(i+1,mod-2)%mod;
  }
  return ans;
}

int main(void){
  scanf("%d%lld",&n,&m);
  ll p=2;
  ll ans=1;
  for(;m>1;p++){
    int e=0;
    while(m%p==0)e++,m/=p;
    ans=ans*f(e)%mod;
  }
  if(m>1)ans=ans*f(1)%mod;
  printf("%lld\n",ans);
}
