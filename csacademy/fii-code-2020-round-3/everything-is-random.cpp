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

const int N = 100100;

ModInt<> a[N];

ll mgk[19] = {1, 1, 3, 13, 73, 501, 4051, 37633, 394353, 4596553, 58941091, 824073141LL, 12470162233LL, 202976401213LL, 3535017524403LL, 65573803186921LL, 1290434218669921LL, 26846616451246353LL, 588633468315403843LL};

VI g[N];
VI ch[N];
int dep[N];
ModInt<> fac[N], invfac[N];
void init_fac(void) {
  fac[0] = 1;
  REP(i, 1, N) fac[i] = fac[i - 1] * i;
  invfac[N - 1] = fac[N - 1].inv();
  for (int i = N - 2; i >= 0; i--) {
    invfac[i] = invfac[i + 1] * (i + 1);
  }
}

ModInt<> sum[N];

void dfs(int v, int par, int d) {
  dep[v] = d;
  for (int w: g[v]) {
    if (par == w) continue;
    ch[v].push_back(w);
    dfs(w, v, d + 1);
  }
}

void get_sum(int v, vector<ModInt<> > &val, ModInt<> a) {
  val[v] += a;
  for (int w: ch[v]) {
    get_sum(w, val, a);
  }
}

void brute(int n) {
  // brute
  REP(i, 0, n) {
    vector<ModInt<> > ans(n);
    VI p(n);
    REP(i, 0, n) p[i] = i;
    do {
      vector<ModInt<> > val(n);
      val[i] = 1;
      REP(j, 0, n) {
        get_sum(p[j], val, val[p[j]]);
      }
      REP(j, 0, n) ans[j] += val[j];
    } while (next_permutation(p.begin(), p.end()));
    cerr << "n = " << n << ", i = " << i << ":";
    REP(j, 0, n) cerr << " " << ans[j];
    cerr << endl;
  }
}

ModInt<> coef[N];

ModInt<> get_coef(int v, int d) {
  ModInt<> s = coef[d];
  for (int w: ch[v]) {
    s += get_coef(w, d + 1);
  }
  return s;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  init_fac();
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    g[u].push_back(v);
    g[v].push_back(u);
  }
  REP(i, 0, n) {
    ll x;
    cin >> x;
    a[i] = x;
  }
  vector<ModInt<> > acc(N);
  acc[0] = 0;
  coef[0] = 2;
  coef[1] = ModInt<>(3) * ModInt<>(2).inv();
  acc[0] = coef[0];
  acc[1] = coef[0] + coef[1];
  REP(i, 2, N) {
    coef[i] = coef[i - 1] + acc[i - 2] * ModInt<>(i + 1).inv();
    acc[i] = acc[i - 1] + coef[i];
  }
  REP(i, 0, 4) cerr << coef[i] * fac[i] << "/" << fac[i] << endl;
  dfs(0, -1, 0);
  // brute
  ModInt<> ans = 0;
  REP(i, 0, n) {
    ans += get_coef(i, 0) * a[i];
  }
  ans *= fac[n];
  cout << ans << endl;
}
