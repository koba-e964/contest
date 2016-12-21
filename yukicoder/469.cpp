#include <cassert>
#include <iostream>
#include <map>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;

ll powmod(ll x, ll e, ll mod) {
  ll sum = 1 % mod;
  ll cur = x % mod;
  while (e > 0) {
    if (e % 2 == 1) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

ll calc(int left, int rr, ll b, ll mod) {
  // TODO naive
  // (b^r-b^l) / (b-1)
  ll sum = powmod(b, rr, mod) - powmod(b, left, mod) + mod;
  sum %= mod;
  sum *= powmod(b, mod - 2, mod);
  sum %= mod;
  return sum;
}

int main(void){
  int n, q;
  cin >> n >> q;
  ll mods[] = {1000000007, 1000000009};
  int nm = sizeof(mods) / sizeof(mods[0]);
  vector<VL> hash(q + 1, VL(nm, 0));
  map<VL, int> res;
  res[hash[0]] = 0;
  REP(i, 0, q) {
    char tt;
    cin >> tt;
    if (tt == '?') {
      REP(j, 0, nm) {
	hash[i + 1][j] = hash[i][j];
      }
    } else {
      int l, r;
      ll k;
      cin >> l >> r >> k;
      REP(j, 0, nm) {
	ll h = calc(l, r, 457, mods[j]);
	h *= mods[j] + k;
	h %= mods[j];
	hash[i + 1][j] = (hash[i][j] + h) % mods[j];
      }
    }
    if (res.count(hash[i + 1]) == 0) {
      res[hash[i + 1]] = i + 1;
    }
    if (tt == '?') {
      cout << res[hash[i + 1]] << endl;
    }
  }
}
