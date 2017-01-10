#include <stdio.h>
#include <stdlib.h>
typedef long long int ll;
const int MOD=1777777777;

ll powmod(ll a, ll e, ll mod) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

ll comb(ll n, int k)
{
  n%=MOD;
  
  ll prod=1;
  for(int i=0; i<k; i++)
    {
      prod *= n - i + MOD;
      prod %= MOD;
      prod *= powmod(i+1, MOD - 2, MOD);
      prod %= MOD;
    }
  return prod%MOD;

}

ll derangement(int k) {
  ll sum = 0;
  ll cur = 1;
  for (int i = 2; i <= k; ++i) {
    cur = cur * powmod(i, MOD - 2, MOD) % MOD;
    sum += cur * (i % 2 == 0 ? 1 : MOD - 1) % MOD;
    sum %= MOD;
  }
  for (int i = 1; i <= k; ++i) {
    sum = sum * i % MOD;
  }
  return sum;
}


int solve(ll n,int k)
{
  ll ret = comb(n, k);
  return ret * derangement(k) % MOD;
}



int main(void)
{
  ll n;
  int k;
  scanf("%lld%d",&n,&k);
  printf("%d\n",(int)solve(n,k));
  return 0;
}
