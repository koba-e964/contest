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


const int N = 110;
ModInt<> dp[N][N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL h(n);
  REP(i, 0, n) cin >> h[i];
  VL coord;
  REP(i, 0, n) {
    if (h[i] >= 2) {
      coord.push_back(h[i]);
    }
  }
  coord.push_back(0);
  sort(coord.begin(), coord.end());
  coord.erase(unique(coord.begin(), coord.end()), coord.end());
  map<ll, int> c_inv;
  REP(i, 0, coord.size()) c_inv[coord[i]] = i;
  VI hc(n);
  REP(i, 0, n) hc[i] = c_inv[h[i]];
  dp[0][0] = 1;
  REP(i, 0, n) {
    // collapse
    if (h[i] == 1) {
      ModInt<> tot;
      REP(j, 0, N) {
	tot = tot + dp[i][j];
      }
      tot = tot * 2;
      dp[i + 1][0] = tot;
      continue;
    }
    // don't care
    if (i == 0 || h[i - 1] == 1) {
      int len = hc[i];
      ModInt<> fac = dp[i][0];
      REP(j, 0, len + 1) {
	dp[i + 1][j] = dp[i + 1][j] + ModInt<>(2).pow(coord[j] == 0 ? h[i] : h[i] - coord[j] + 1);
	if (j < len) {
	  dp[i + 1][j] = dp[i + 1][j] - ModInt<>(2).pow(coord[j + 1] == 0 ? h[i] : h[i] - coord[j + 1] + 1);
	}
	dp[i + 1][j] = dp[i + 1][j] * fac;
      }
      continue;
    }
    int u = hc[i - 1];
    int v = hc[i];
    int m = min(u, v);
    ModInt<> fac = ModInt<>(2).pow(v >= u ? coord[v] - coord[u] : 0);
    assert (u >= 1);
    assert (v >= 1);
    if (DEBUG) {
      DEBUGP(u);
      DEBUGP(v);
      DEBUGP(m);
      DEBUGP(fac);
    }
    REP(j, 0, m) {
      dp[i + 1][j] = dp[i + 1][j] + dp[i][j] * fac;
    }
    if (u >= v) {
      REP(j, v, u + 1) {
	dp[i + 1][v] = dp[i + 1][v] + dp[i][j] * 2;
      }
    } else {
      REP(j, u, v + 1) {
	ModInt<> tmp = ModInt<>(2).pow(h[i] - coord[j]);
	if (j < v) {
	  tmp = tmp - ModInt<>(2).pow(h[i] - coord[j + 1]);
	}
	dp[i + 1][j] = dp[i + 1][j] + tmp * dp[i][u] * 2;
      }
    }
  }
  if (DEBUG) {
    cerr << "dp:" << endl;
    REP(i, 0, n + 1) {
      REP(j, 0, n + 1) {
	cerr << " " << dp[i][j];
      }
      cerr << endl;
    }
  }
  ModInt<> tot;
  REP(i, 0, N) tot = tot + dp[n][i];
  cout << tot << endl;
}
