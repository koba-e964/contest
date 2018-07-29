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

const int N = 40;
const int T = 10000;
ModInt<> dp[N][N][N];

ModInt<> fac[N], invfac[N];
ModInt<> p2[T];
void init(void) {
  fac[0] = 1;
  REP(i, 1, N) {
    fac[i] = fac[i - 1] * i;
  }
  invfac[N - 1] = fac[N - 1].inv();
  for (int i = N - 2; i >= 0; --i) {
    invfac[i] = invfac[i + 1] * (i + 1);
  }
  p2[0] = 1;
  REP(i, 1, T) {
    p2[i] = p2[i - 1] * 2;
  }
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, d;
  cin >> n >> d;
  dp[0][1][1] = 1;
  REP(i, 1, d + 1) {
    REP(j, 0, N) {
      REP(k, 0, N) {
	REP(l, 1, N - j) {
	  ModInt<> &ret = dp[i][j + l][l];
	  ModInt<> tmp = dp[i - 1][j][k];
	  tmp = tmp * p2[l * (l - 1) / 2];
	  tmp = tmp * (p2[k] - 1).pow(l);
	  tmp = tmp * invfac[i == d ? l - 1 : l];
	  ret = ret + tmp;
	}
      }
    }
  }
  if (DEBUG) {
    REP(i, 0, d + 1) {
      cerr << "dp[" << i << "]:";
      REP(j, 0, n + 1) {
	REP(k, 0, n + 1) {
	  cerr << " " << dp[i][j][k];
	}
	cerr << endl;
      }
    }
  }
  ModInt<> tot(0);
  REP(sum, 2, n + 1) {
    REP(last, 1, n) {
      ModInt<> tmp = dp[d][sum][last];
      int rem = n - sum;
      tmp = tmp * p2[last * (n - sum)];
      tmp = tmp * invfac[n - sum];
      tmp = tmp * p2[rem * (rem - 1) / 2];
      tot = tot + tmp;
    }
  }
  tot = tot * fac[n - 2];
  cout << tot << endl;
}
