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


/*
 * Dependencies: typedef long long ll
 * Headers: iostream
 * Verified by: ARC099-F
 *              https://beta.atcoder.jp/contests/arc099/submissions/2743855
 */

template<ll mod = (ll)998244353>
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

VI g[4000];
int w[4000];
int K;

vector<ModInt<>> dfs(int v, int par, ModInt<> &add) {
  int S = 1;
  vector<ModInt<>> dp(2 * S + 1);
  dp[S + w[v]] = 1;
  for (auto t: g[v]) {
    if (t != par) {
      auto res = dfs(t, v, add);
      int T = res.size() / 2;
      auto U = min(S + T, K);
      auto ndp = vector<ModInt<>>(2 * U + 1);
      
      for (int i = -S; i <= S; i++) {
        ndp[i + U] += dp[i + S];
        for (int j = -T; j <= T; j++) {
          auto k = i + j + U;
          if (0 <= k && k < (int) ndp.size()) ndp[k] += dp[i + S] * res[j + T];
        }
      }
      dp = ndp;
      S = U;
    }
  }
  for (int i = 1; i <= S; i++) add += dp[S + i];
  return dp;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI c(n);
  REP(i, 0, n) cin >> c[i], c[i]--;
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    g[a].push_back(b);
    g[b].push_back(a);
  }
  ModInt<> ans(0);
  VI pop(n);
  REP(i, 0, n) {
    pop[c[i]]++;
  }
  REP(i, 0, n) w[i] = -1;
  REP(col, 0, n) {
    if (pop[col] == 0) continue;
    K = pop[col];
    REP(i, 0, n) if (c[i] == col) w[i] = 1;
    ModInt<> add(0);
    dfs(0, -1, add);
    ans += add;
    REP(i, 0, n) if (c[i] == col) w[i] = -1;
  }
  cout << ans << endl;
}
