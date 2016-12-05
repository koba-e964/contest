#include <vector>
#include <cmath>
#include <map>
#include <stdio.h>
#include <iostream>
#include <cassert>

using namespace std;
typedef long long int i64;
typedef long long ll;
#define REP(i, s, n) for (int i=(s); i <(n);++i)

ll invmod(ll a,ll p)
{
  ll pow=p-2;
  i64 sum=1;
  i64 cur=a;
  while(pow>0)
    {
      if(pow&1)
	{
	  sum*=cur;
	  sum%=p;
	}
      cur*=cur;
      cur%=p;
    }
  return sum;
}

ll powmod(ll x, ll e, ll mod) {
  ll sum = 1;
  ll cur = x % mod;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

const int T = 1000010;
int t;
ll comb_tbl[T];
ll comb(int x) {
  if (x < 0 || x > t) {
    return 0;
  }
  return comb_tbl[x];
}

void pre_compute(int t, ll mod) {
  // factorize mod
  map<int, int> fact;
  {
    int tv = mod;
    int p = 2;
    while (tv > 1 && p * p <= mod) {
      int cnt = 0;
      while (tv % p == 0) {
	cnt++;
	tv /= p;
      }
      if (cnt > 0) {
	fact[p] = cnt;
      }
      p++;
    }
    if (tv >= 2) {
      fact[tv] = 1;
    }
  }
  int m = fact.size();
  ll totient = 1;
  vector<pair<int, int> > fv(fact.begin(), fact.end());
  REP(i, 0, m) {
    ll p = fv[i].first;
    totient *= p - 1;
    REP(j, 1, fv[i].second) {
      totient *= p;
    }
  }
  // compute comb(t, i)
  vector<int> pexp(m, 0);
  ll cur = 1;
  for (int i = 0; i < t; ++i) {
    ll tmp = cur;
    REP(i, 0, m) {
      ll p = fv[i].first;
      REP(j, 0, pexp[i]) {
	tmp = tmp * p % mod;
      }
    }
    comb_tbl[i] = tmp;
    int v = i + 1, w = t - i;
    REP(i, 0, m) {
      int p = fv[i].first;
      while (v % p == 0) {
	v /= p;
	pexp[i]--;
      }
      while (w % p == 0) {
	w /= p;
	pexp[i]++;
      }
    }
    // get the inverse of v mod mod
    v %= mod;
    w %= mod;
    cur = cur * powmod(v, totient - 1, mod) % mod;
    cur = cur * w % mod;
  }
  comb_tbl[t] = 1;
}

int main(void)
{
  int n;
  ll mod;
  cin >> n >> t >> mod;
  ll sum = 1;
  pre_compute(t, mod);
  for(int i=0;i<n;i++) {
    ll x, y;
    cin >> x >> y;
    if (((t + x + y) % 2 + 2) % 2 == 1) {
      sum = 0;
    }
    if (abs(x) + abs(y) > t) {
      sum = 0;
    }
    ll tmp = comb((t - x - y) / 2) * comb((t - x + y) / 2) % mod;
    sum = sum * tmp % mod;
  }
  cout << sum << endl;
}
