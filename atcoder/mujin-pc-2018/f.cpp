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
const ll mod = 998244353;

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

const int N = 2010;
ModInt<> dp[N][N];

ModInt<> comb[N][N];
void init(void) {
  comb[0][0] = 1;
  REP(i, 1, N) {
    REP(j, 0, N) {
      comb[i][j] = comb[i][j] + comb[i - 1][j];
      if (j > 0) {
	comb[i][j] = comb[i][j] + comb[i - 1][j - 1];
      }
    }
  }
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) cin >> a[i];
  sort(a.begin(), a.end());
  dp[n][0] = 1;
  // TODO n^3
  for (int i = n - 1; i >= 0; --i) {
    dp[i][0] = 1;
    REP(j, 1, n - i + 1) {
      dp[i][j] = dp[i][j] + dp[i + 1][j];
      REP(l, 1, min(a[i], j) + 1) {
	ModInt<> nxt = dp[i + 1][j - l];
	dp[i][j] = dp[i][j] + nxt * comb[n - i - 1 - j + l][l - 1];
      }
    }
  }
  if (DEBUG) {
    REP(i, 0, n + 1) {
      REP(j, 0, n + 1) {
	cerr << " " << dp[i][j];
      }
      cerr << endl;
    }
  }
  cout << dp[0][n] << endl;
}
