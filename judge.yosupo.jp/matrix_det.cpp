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
const ll mod = 998244353;

/*
 * Dependencies: typedef long long ll
 * Headers: iostream
 * Verified by: ARC099-F
 *              https://beta.atcoder.jp/contests/arc099/submissions/2743855
 */

template<ll mod = mod>
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


ModInt<> get_det(vector<vector<ModInt<> > > &A) {
  ModInt<> det(1);
  int n = A.size();
  REP(i, 0, n) {
    int r2 = n;
    REP(j, i, n) {
      if (A[j][i].to_ll() != 0) {
        r2 = j;
        break;
      }
    }
    if (r2 >= n) return 0;
    if (r2 != i) {
      swap(A[i], A[r2]);
      det = -det;
    }
    ModInt<> a = A[i][i].inv();
    det *= A[i][i];
    A[i][i] = 1;
    REP(j, i + 1, n) {
      A[i][j] *= a;
    }
    REP(j, i + 1, n) {
      ModInt<> aji = A[j][i];
      A[j][i] = 0;
      REP(k, i + 1, n) {
	A[j][k] -= aji * A[i][k];
      }
    }
  }
  return det;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  vector<vector<ModInt<> > > mat(n, vector<ModInt<> >(n));
  REP(i, 0, n) {
    REP(j, 0, n) {
      ll x;
      cin >> x;
      mat[i][j] = x;
    }
  }
  ModInt<> det = get_det(mat);
  cout << det << endl;
}
