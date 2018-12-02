#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>
#include <set>

using namespace std;
const int DEBUG = 0;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

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

const int N = 3100;
VI g[N];
const int B = 2;
ModInt<> b[B];

vector<ModInt<> > dfs_hash(int v, int par, const vector<bool> &present) {
  vector<ModInt<> > hash(2, 1);
  for (int w: g[v]) {
    if (par == w) { continue; }
    if (!present[w]) { continue; }
    vector<ModInt<> > sub = dfs_hash(w, v, present);
    REP(i, 0, B) {
      hash[i] += b[i] * sub[i].pow(2);
    }
  }
  return hash;
}

void dfs(int v, int par, vector<int> &dist, int d) {
  dist[v] = d;
  for (int w: g[v]) {
    if (par == w) { continue; }
    dfs(w, v, dist, d + 1);
  }
}


int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n, r;
  cin >> n >> r;
  b[0] = 0xef540f10; b[1] = 0x899ada48;
  REP(i, 0, n - 1) {
    int u, v;
    cin >> u >> v;
    u--, v--;
    g[u].push_back(v);
    g[v].push_back(u);
  }
  set<vector<ModInt<> > > hashes;
  REP(i, 0, n) {
    VI d0(n);
    dfs(i, -1, d0, 0);
    PI ma(0, i);
    vector<bool> present(n);
    REP(i, 0, n) present[i] = d0[i] <= r;
    REP(j, 0, n) {
      if (present[j])
        ma = max(ma, PI(d0[j], j));
    }
    if (DEBUG) {
      cerr << i << " d0:";
      REP(j, 0, n) cerr << " " << d0[j];
      cerr << endl;
    }
    int v1 = ma.second;
    VI d1(n);
    dfs(v1, -1, d1, 0);
    ma = PI(0, v1);
    REP(i, 0, n) {
      if (present[i]) ma = max(ma, PI(d1[i], i));
    }
    if (DEBUG) {
      cerr << i << " d1:";
      REP(j, 0, n) if (present[j]) cerr << " " << d1[j]; else cerr << " -";
      cerr << endl;
    }
    int v2 = ma.second;
    int diam = ma.first;
    int rem = diam;
    int cur = v2;
    int p1 = -1, p2 = -1;
    while (rem >= 0) {
      int nxt = -1;
      if (diam % 2 == 0) {
        if (rem == diam / 2) {
          p1 = cur;
          p2 = cur;
        }
      } else {
        if (rem == diam / 2 + 1) p1 = cur;
        if (rem == diam / 2) p2 = cur;
      }
      if (rem == 0) break;
      for (int w: g[cur]) {
        if (not present[w]) continue;
        if (d1[w] == rem - 1) {
          nxt = w;
          break;
        }
      }
      rem--;
      cur = nxt;
    }
    assert (present[p1]);
    assert (present[p2]);
    if (DEBUG) {
      cerr << "p: " << p1 << " " << p2 << endl;
    }
    vector<ModInt<> > hash(2);
    vector<ModInt<> > sub = dfs_hash(p1, -1, present);
    REP(j, 0, B) hash[j] += sub[j];
    sub = dfs_hash(p2, -1, present);
    REP(j, 0, B) hash[j] += sub[j];
    hashes.insert(hash);
  }
  cout << hashes.size() << endl;
}
