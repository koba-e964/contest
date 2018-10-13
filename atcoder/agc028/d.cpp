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


int isnotcut(int l, int r, int a, int b) {
  if (a > b) swap(a, b);
  if (l <= a && b < r) return +1;
  if (a < l && r <= b) return -1;
  if (b < l) return -1;
  if (r <= a) return -1;
  return 0;
}


const int N = 1010;
ModInt<> fac[N], invfac[N];
void init() {
  fac[0] = 1;
  REP(i, 1, N) fac[i] = fac[i - 1] * i;
  invfac[N - 1] = fac[N - 1].inv();
  for (int i = N - 2; i >= 0; --i) invfac[i] = invfac[i + 1] * (i + 1);
}

ModInt<> dp[N][N], dpo[N][N];

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  VI a(k), b(k);
  REP(i, 0, k) {
    cin >> a[i] >> b[i];
    a[i]--, b[i]--;
    if (a[i] > b[i])
      swap(a[i], b[i]);
  }
  vector<ModInt<> > tbl(n + 1);
  REP(i, 0, n + 1) {
    tbl[i] = fac[2 * i] * invfac[i] * invfac[2].pow(i);
  }
  ModInt<> tot;
  REP(s, 2, 2 * n) {
    REP(i, 0, 2 * n - s) {
      int j = s + i;
      if ((j - i) % 2 != 0) continue;
      bool ok = 1;
      // All edges are not cut?
      int u = 0, v = 0;
      REP(l, 0, k) {
        int val = isnotcut(i, j, a[l], b[l]);
        if (val == 0) {
          ok = 0;
          break;
        }
        if (val == 1) u++;
        else v++;
      }
      if (not ok) continue;
      u = (j - i) / 2 - u;
      v = n - (j - i) / 2 - v;
      assert (u >= 0);
      assert (v >= 0);
      assert (u + v == n - k);
      if (0) {
        DEBUGP(i);
        DEBUGP(j);
        DEBUGP(u);
        DEBUGP(v);
        cerr << endl;
      }
      dp[i][j] = dpo[i][j] = tbl[u];
      dp[j][i] = dpo[j][i] = tbl[v];
    }
  }
  REP(s, 2, 2 * n) {
    REP(i, 0, 2 * n - s) {
      int j = s + i;
      if ((j - i) % 2 != 0) continue;
      REP(k, i, j) {
        dp[i][j] -= dpo[i][k] * dp[k][j];
        // dp[j][i] -= dp[j][k] * dp[k][i];
      }
      tot += dp[i][j] * dpo[j][i];
    }
  }
  for (int s = 2 * n - 2; s >= 2; s -= 2) {
    REP(i, 0, 2 * n - s) {
      int j = s + i;
      if ((j - i) % 2 != 0) continue;
      if (0) {
        DEBUGP("orig");
        DEBUGP(dp[j][i]);
      }
      REP(k, 0, 2 + n) {
        if (k < i || j < k) {
          dp[j][i] -= dpo[j][k] * dp[k][i];
        }
      }
      // tot += dp[i][j] * dp[j][i];
      if (0) {
        cerr << "(i,j)=" << i << " " << j << endl;
        DEBUGP(dp[i][j]);
        DEBUGP(dp[j][i]);
        DEBUGP(tot);
      }
    }
  }
  tot += tbl[n - k];
  cout << tot << endl;
}
