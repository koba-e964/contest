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

const int N = 3010;
ModInt<> dp[N][N];
ModInt<> acc[N][N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n >> q;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  VI x(q), y(q);
  REP(i, 0, q) {
    cin >> x[i] >> y[i];
    x[i]--, y[i]--;
  }
  REP(i, 0, n) {
    dp[i][i] = 1;
  }
  ModInt<> two(2);
  ModInt<> twoinv = two.inv();
  REP(i, 0, q) {
    REP(j, 0, n) {
      ModInt<> tmp = (dp[j][x[i]] + dp[j][y[i]]) * twoinv;
      dp[j][x[i]] = tmp;
      dp[j][y[i]] = tmp;
    }
  }
  REP(i, 0, n) {
    REP(j, 0, n) acc[i][j + 1] = acc[i][j] + dp[i][j];
  }
  vector<PI> pool;
  REP(i, 0, n) pool.push_back(PI(a[i], i));
  sort(pool.rbegin(), pool.rend());
  vector<ModInt<> > tap(n + 1);
  VI que;
  // TODO proof
  ModInt<> tot = 0;
  REP(i, 0, n) {
    if (DEBUG) cerr << "pool:" << pool[i].first << " " << pool[i].second << endl;
    if (i > 0 && pool[i].first != pool[i - 1].first) {
      // tap
      for (int t: que) {
        if (DEBUG) cerr << "pushing " << t << endl;
        REP(j, 0, n + 1) {
          tap[j] += acc[t][j];
        }
      }
      que.clear();
    }
    int idx = pool[i].second;
    REP(k, 0, n) {
      tot += dp[idx][k] * tap[k];
      tot += dp[idx][k] * (tap[k + 1] - tap[k]) * twoinv;
    }
    que.push_back(idx);
  }
  REP(i, 0, q) tot *= two;
  cout << tot << endl;
}
