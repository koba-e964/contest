#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>

const int DEBUG = 0;

using namespace std;
typedef long long ll;
/*
 * Dependencies: typedef long long ll
 * Headers: iostream
 * Verified by: ARC099-F
 *              https://beta.atcoder.jp/contests/arc099/submissions/2743855
 */

template<ll mod = (ll)1e9 + 9>
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


#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


template<class T>
vector<vector<T> > mul(const vector<vector<T> > &a, const vector<vector<T> > &b)
{
  int n = a.size();
  vector<vector<T> > ret(n, vector<T>(n));
  REP(i, 0, n) {
    REP(j, 0, n) {
      REP(k, 0, n) {
        ret[i][k] += a[i][j] * b[j][k];
      }
    }
  }
  return ret;
}
template<class T>
vector<vector<T> > pw(const vector<vector<T> > &a, ll e) {
  int n = a.size();
  vector<vector<T> > prod(n, vector<T>(n));
  REP(i, 0, n) prod[i][i] = 1;
  vector<vector<T> > cur(a);
  while (e > 0) {
    if (e % 2) {
      prod = mul(prod, cur);
    }
    cur = mul(cur, cur);
    e /= 2;
  }
  return prod;
}

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  ll s;
  int m, d;
  cin >> s >> m >> d;
  int dim = m * m;
  vector<vector<ModInt<> > > mat(dim, vector<ModInt<> >(dim));
  REP(i, 0, m) {
    REP(j, 0, m) {
      if (abs(i - j) > d) continue;
      mat[(m - 1) * m + j][(m - 1 - j) * m + i] += 1;
    }
  }
  REP(i, 0, m - 1) {
    REP(j, 0, m) {
      mat[i * m + j][(i + 1) * m + j] = 1;
    }
  }
  ll init = max(s - m, 0LL);
  vector<vector<ModInt<> > > ans = pw(mat, init);
  if (DEBUG) {
    cerr << "mat:" << endl;
    REP(i, 0, dim) {
      REP(j, 0, dim) {
        cerr << " " << mat[i][j];
      }
      cerr << endl;
    }
    cerr << "ans:" << endl;
    REP(i, 0, dim) {
      REP(j, 0, dim) {
        cerr << " " << ans[i][j];
      }
      cerr << endl;
    }
  }
  ModInt<> tot;
  for (ll e = init; e < s; ++e) {
    ll i = s - e - 1;
    if (i >= 0 && i < m) {
      REP(j, 0, m) {
        tot += ans[(m - 1) * m + j][(m - 1) * m + i];
      }
    }
    ans = mul(ans, mat);
    if (DEBUG) {
      cerr << "mat^" << e + 1 << ":" << endl;
      REP(i, 0, dim) {
        REP(j, 0, dim) {
          cerr << " " << ans[i][j];
        }
        cerr << endl;
      }
    }
  }
  cout << tot << endl;
}
