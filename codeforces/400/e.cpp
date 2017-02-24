#include <iostream>
#include <map>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const ll mod = 1e9 + 7;


void factorize(long long v, std::map<long long, int> &result) {
  long long p = 2;
  while (v > 1 && p * p <= v) {
    int cnt = 0;
    while (v % p == 0) {
      cnt++;
      v /= p;
    }
    if (cnt > 0) {
      if (result.count(p) == 0) {
	result[p] = 0;
      }
      result[p] += cnt;
    }
    p += p == 2 ? 1 : 2;
  }
  if (v > 1) {
    if (result.count(v) == 0) {
      result[v] = 0;
    }
    result[v] += 1;
  }
}

ll totient(ll x) {
  map<ll, int> res;
  factorize(x, res);
  ll ret = 1;
  for (auto e: res) {
    ll a = e.first;
    int b = e.second;
    REP(i, 0, b - 1) {
      ret *= a;
    }
    ret *= a - 1;
  }
  return ret;
}


ll calc(ll n, ll k) {
  ll cur = n;
  // This loop will not continue so long; cur decreases quickly.
  for (ll i = 0; i < k; ++i) {
    ll res = totient(cur);
    cur = res;
    if (cur == 1) {
      break;
    }
  }
  return cur % mod;
}


int main(void){
  ll n, k;
  cin >> n >> k;
  k = (k + 1) / 2;
  // I want f^k(n)!
  cout << calc(n, k) << endl;
}
