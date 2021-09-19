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
const ll mod = 998244353;

/*
 * Dependencies: typedef long long ll
 * Headers: iostream
 * Verified by: ARC099-F
 *              https://beta.atcoder.jp/contests/arc099/submissions/2743855
 */

template<ll mod = (ll)mod>
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

inline bool mtch(char c, char d) {
  return c == d || c == '?';
}

// s = BA.A., t = CA.A.
// ... of t = flip(... of s)
ModInt<> pat_1(int n, const string &s, const string &t, char a, char b, char c) {
  int tot = 0;
  REP(i, 0, n) if (!mtch(s[2 * i + 1], a)) return 0;
  REP(i, 0, n) if (!mtch(t[2 * i + 1], a)) return 0;
  if (!mtch(s[0], b)) return 0;
  if (!mtch(t[0], c)) return 0;
  vector<bool> l(n + 2), r(n + 2);
  l[1] = true;
  r[n + 1] = true;
  REP(i, 1, n + 1) {
    l[i + 1] = l[i] && mtch(s[2 * i], b) && mtch(t[2 * i], c);
  }
  for (int i = n; i >= 0; i--) {
    r[i] = r[i + 1] && mtch(s[2 * i], c) && mtch(t[2 * i], b);
  }
  REP(i, 1, n + 2) {
    if (l[i] && r[i]) tot++;
  }
  return tot;
}

// s = ABABA, t = BCBCB
ModInt<> pat_2(int n, const string &s, const string &t, char a, char b, char c) {
  REP(i, 0, n) if (!mtch(s[2 * i + 1], b)) return 0;
  REP(i, 0, n) if (!mtch(t[2 * i + 1], c)) return 0;
  REP(i, 0, n + 1) if (!mtch(s[2 * i], a)) return 0;
  REP(i, 0, n + 1) if (!mtch(t[2 * i], b)) return 0;
  return 1;
}

// s = ABABA, t = C.C.C
// ... cannot be all equal
ModInt<> pat_3(int n, const string &s, const string &t, char a, char b, char c) {
  REP(i, 0, n) if (!mtch(s[2 * i + 1], b)) return 0;
  REP(i, 0, n + 1) if (!mtch(s[2 * i], a)) return 0;
  REP(i, 0, n + 1) if (!mtch(t[2 * i], c)) return 0;
  ModInt<> dp[3] = {0, 0, 0};
  if (mtch(t[1], a)) dp[1] = 1;
  if (mtch(t[1], b)) dp[2] = 1;
  REP(i, 1, n) {
    ModInt<> ep[3] = {0, 0, 0};
    if (mtch(t[2 * i + 1], a)) {
      ep[0] += dp[0] + dp[2];
      ep[1] += dp[1];
    }
    if (mtch(t[2 * i + 1], b)) {
      ep[0] += dp[0] + dp[1];
      ep[2] += dp[2];
    }
    REP(i, 0, 3) dp[i] = ep[i];
  }
  return dp[0];
}
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  while (t--) {
    int n;
    cin >> n;
    string s, t;
    cin >> s >> t;
    ModInt<> tot(0);
    string per = "ABC";
    do {
      tot += pat_1(n, s, t, per[0], per[1], per[2]);
      tot += pat_2(n, s, t, per[0], per[1], per[2]);
      tot += pat_2(n, t, s, per[0], per[1], per[2]);
      tot += pat_3(n, s, t, per[0], per[1], per[2]);
      tot += pat_3(n, t, s, per[0], per[1], per[2]);
    } while (next_permutation(per.begin(), per.end()));
    cout << tot << "\n";
  }
}
