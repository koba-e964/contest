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


#if 1

const int N = 200010;
ModInt<> fac[N], invfac[N], inv[N];
void init() {
  fac[0] = 1;
  REP(i, 1, N) fac[i] = fac[i - 1] * i;
  invfac[N - 1] = fac[N - 1].inv();
  for (int i = N - 2; i >= 0; --i) invfac[i] = invfac[i + 1] * (i + 1);
  REP(i, 1, N) {
    inv[i] = invfac[i] * fac[i - 1];
  }
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  vector<ModInt<> > c(n);
  ModInt<> t;
  REP(i, 0, n) {
    t += inv[i + 1];
  }
  c[0] = t;
  REP(i, 1, n) {
    t += inv[i + 1];
    t -= inv[n - i + 1];
    c[i] = t;
  }
  REP(i, 0, n) c[i] *= fac[n];
  ModInt<> ans;
  REP(i, 0, n) ans += c[i] * a[i];
  cout << ans << endl;
}
#else
int main(void) {
  int n;
  cin >> n;
  VI p(n);
  REP(i, 0, n) p[i] = i;
  VL t(n);
  do {
    VI u(n, 1);
    REP(i, 0, n) {
      int l = p[i];
      int r = p[i];
      while (l >= 0) {
        if (u[l] == 0) {
          break;
        }
        l--;
      }
      l++;
      while (r < n) {
        if (u[r] == 0) {
          break;
        }
        r++;
      }
      r--;
      REP(j, l, r + 1) t[j]++;
      u[p[i]] = 0;
    }
  } while (next_permutation(p.begin(), p.end()));
  REP(i, 0, n) cerr << " " << t[i];
  cerr << endl;
}
#endif
