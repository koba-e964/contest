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

const int DEBUG = 0;

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

const int N = 1000100;
VI g[N];
ll a[N];

VL ew;
ll dfs(int v) {
  ll tot = a[v];
  for (int w: g[v]) {
    ll sub = dfs(w);
    tot += sub;
  }
  ew.push_back(tot);
  return tot;
}

int main(void) {
  int n;
  scanf("%d", &n);
  REP(i, 0, n) scanf("%lld", &a[i]);
  REP(i, 1, n) {
    int p;
    scanf("%d", &p);
    p--;
    g[p].push_back(i);
  }
  ll asum = dfs(0);
  assert ((int) ew.size() == n);
  sort(ew.begin(), ew.end());
  VL es(ew.begin(), ew.end());
  es.erase(unique(es.begin(), es.end()), es.end());
  VL divs;
  for (ll e: es) {
    if (asum % e == 0 && asum / e <= n) {
      divs.push_back(e);
    }
  }
  VL dpi;
  for (ll e: divs) {
    ll count = 0;
    ll qq = asum / e;
    for (ll j = 1; j <= qq; ++j) {
      ll val = e * j;
      int u = upper_bound(ew.begin(), ew.end(), val) - lower_bound(ew.begin(), ew.end(), val);
      count += u;
    }
    if (count == qq) {
      dpi.push_back(e);
    }
  }
  if (DEBUG) {
    cerr << "dpi:";
    for (ll e: dpi) cerr << " " << e;
    cerr << endl;
  }
  int m = dpi.size();
  vector<ModInt<> > dp(m);
  dp[m - 1] = 1;
  for (int i = m - 2; i >= 0; --i) {
    REP(j, i + 1, m) {
      if (dpi[j] % dpi[i] == 0) {
        dp[i] += dp[j];
      }
    }
  }
  ModInt<> tot(0);
  REP(i, 0, m) {
    tot += dp[i];
  }
  printf("%lld\n", tot.x);
}
