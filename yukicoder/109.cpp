#include <cassert>
#include <iostream>
#include <map>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

ll powmod(ll x, ll e, ll mod) {
  ll sum = 1 % mod;
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

ll solve(ll n, ll m) {
  if (n >= m) {
    return 0;
  }
  if (m == 1) {
    return 0;
  }
  // factorize m
  map<ll, int> fact;
  {
    ll p = 2;
    ll v = m;
    while (v > 1 && p * p <= v) {
      int cnt = 0;
      while (v % p == 0) {
	v /= p;
	cnt++;
      }
      if (cnt > 0) {
	fact[p] = cnt;
      }
      if (p == 2) { p = 3; }
      else { p += 2; }
    }
    if (v > 1) {
      fact[v] = 1;
    }
  }
  if (fact.count(m) == 1) { // m is a prime
    ll sum = m - 1;
    REP(i, 1, m - n) {
      sum = sum * powmod(m - i, m - 2, m) % m;
    }
    return sum;
  }
  if (m >= 300000) {
    return 0;
  }
  ll sum = 1;
  REP(i, 1, n + 1) {
    sum = sum * i % m;
  }
  return sum;
}

int main(void){
  int t;
  cin >> t;
  while (t--) {
    ll m, n;
    cin >> n >> m;
    cout << solve(n, m) << endl;
  }
}
