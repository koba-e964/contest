#include <cstdio>
typedef long long ll;
ll m=1e9+7,f[1<<21],n,k,t,s,u,e;char c;
ll r(ll v){for(e=m-2,s=1,u=v%m;e;u=u*u%m,e/=2)if(e%2)s=s*u%m;return s;}
ll b(ll n,ll k){return n<k?0:f[n]*r(f[k])%m*r(f[n-k])%m;}
int main(){for(f[t=0]=1;++t<1<<21;f[t]=f[t-1]*t%m);for(scanf("%lld",&t);t--;printf("%lld\n",c%2?b(n,k):c%5?(n?b(n+k-1,k):!k):n>=k?f[n]*r(f[n-k])%m:0))scanf("%*[\n]%c(%lld,%lld)",&c,&n,&k);}
