#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;
/*
 * Dependencies: typedef long long ll
 * Headers: iostream
 * Verified by: ARC099-F
 *              https://beta.atcoder.jp/contests/arc099/submissions/2743855
 */

template<ll mod = (ll)1e9 + 7>
struct ModInt {
  ll x;
  ModInt(void): x(0) {}
  ModInt(ll x): x(x % mod){}
  ModInt(const ModInt &x): x(x.x) {}
  ModInt operator+(ModInt o) const {
    ll y = x + o.x;
    if (y >= mod) y -= mod;
    return ModInt(y);
  }
  ModInt operator-(ModInt o) const {
    ll y = x - o.x + mod;
    if (y >= mod) y -= mod;
    return ModInt(y);
  }
  ModInt operator*(ModInt o) const {
    return ModInt((x * o.x) % mod);
  }
  void operator+=(ModInt o) { *this = *this + o; }
  void operator-=(ModInt o) { *this = *this - o; }
  void operator*=(ModInt o) { *this = *this * o; }
  ModInt operator-(void) const { return ModInt() - *this; }
  ll to_ll() const {
    return x;
  }
  bool operator<(ModInt o) const {
    return x < o.x;
  }
  ModInt pow(ll e) {
    assert (e >= 0);
    ModInt sum = 1;
    ModInt cur = *this;
    while (e > 0) {
      if (e % 2) {
        sum = sum * cur;
      }
      cur = cur * cur;
      e /= 2;
    }
    return sum;
  }
  ModInt inv(void) {
    return pow(mod - 2);
  }
};

template<ll mod>
ostream &operator<<(ostream &os, ModInt<mod> mi) {
  return os << mi.x;
}


#define TEST 0

#if TEST

int main(void) {
  int n;
  cin >> n;
  VI a(n - 1);
  REP(i, 0, n - 1) a[i] = i;
  VI ans(n);
  do {
    int sc = 0;
    VI tmp(n);
    REP(i, 0, n - 1) {
      sc += 1 - tmp[a[i]];
      tmp[a[i]] = 1;
      sc += 1 - tmp[a[i] + 1];
      tmp[a[i] + 1] = 1;
      if (sc == n) {
	ans[i + 1] += 1;
	break;
      }
    }
  } while (next_permutation(a.begin(), a.end()));
  ll fac = 1;
  REP(i, 0, (n + 1) / 2) {
    fac *= i + 1;
  }
  cerr << "factor = " << fac << endl;
  REP(i, 0, n) {
    cerr << " " << ans[i] / fac;
  }
  cerr << endl;
}


#else

const int N = 1001000;
ModInt<> fac[N], invfac[N];
void init(void) {
  fac[0] = 1;
  REP(i, 1, N) fac[i] = fac[i - 1] * i;
  invfac[N - 1] = fac[N - 1].inv();
  for (int i = N - 2; i >= 0; --i) invfac[i] = invfac[i + 1] * (i + 1);
}

ModInt<> comb(int x, int y) {
  assert (x >= 0);
  if (y < 0 || y > x) return 0;
  return fac[x] * invfac[y] * invfac[x - y];
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n;
  cin >> n;
  // Place "10" * (n - 1 - i) and "1" * (2 * i - n), and place "1", forming a string of length n - 1
  vector<ModInt<> > a(n);
  REP(i, 1, n) {
    a[i] = comb(i - 1, n - 1 - i) * fac[i] * fac[n - i - 1];
  }
  for (int i = n - 1; i >= 1; --i) a[i] -= a[i - 1];
  ModInt<> tot;
  REP(i, 0, n) {
    tot += a[i] * i;
  }
  cout << tot << endl;
}

#endif
