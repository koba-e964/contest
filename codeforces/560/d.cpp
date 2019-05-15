#include <stdio.h>
#include <algorithm>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

typedef long long ll;

int t,n;
ll d[310];
ll u[310000];
int uptr;

int main(void){
  scanf("%d",&t);
  while(t--){
    scanf("%d",&n);
    REP(i,0,n)scanf("%lld",d+i);
    sort(d,d+n);
    ll ans=d[0]*d[n-1];

    uptr=0;
    for(ll i=1;i*i<=ans;++i){
      if(ans%i==0){
        u[uptr++]=i;
        if(ans!=i*i)u[uptr++]=ans/i;
      }
    }
    sort(u,u+uptr);
    int ok=uptr==n+2;
    if(ok){
      REP(i,0,n)ok&=u[i+1]==d[i];
    }
    printf("%lld\n",ok?ans:-1);
  }
}
