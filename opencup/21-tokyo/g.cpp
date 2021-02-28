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

const int LEN = 823543; // 7^7

// a[i] をフーリエ変換
// i は 7 桁の 7 進数
void fourier(vector<ModInt<>> &a, ModInt<> zeta) {
  int base = 1; // base = pow(7, i)
  vector<ModInt<>> tmp(7);
  REP(i, 0, 7) {
    for (int k = 0; k < LEN; k += 7 * base) {
      REP(j, k, k + base) {
        REP(l, 0, 7) tmp[l] = 0;
        ModInt<> fst = 1;
        REP(l, 0, 7) {
          ModInt<> cur = 1;
          REP(u, 0, 7) {
            tmp[l] += a[j + u * base] * cur;
            cur *= fst;
          }
          fst *= zeta;
        }
        REP(l, 0, 7) a[j + l * base] = tmp[l];
      }
    }
    base *= 7;
  }
}

// 7 進数の畳み込み
// ret[i ^ j] += a[i] * b[j]
// i, j は 7 桁の 7 進数
vector<ModInt<>> conv(const vector<ModInt<>> &a, const vector<ModInt<>> &b) {
  vector<ModInt<>> ap(a), bp(b);
  ModInt<> o7 = ModInt<>(3).pow((mod - 1) / 7);
  fourier(ap,o7);
  fourier(bp,o7);
  REP(i, 0, LEN) ap[i] *= bp[i];
  fourier(ap,o7.inv());
  ModInt<> fac = ModInt<>(7).pow(mod - 8);
  REP(i, 0, LEN) ap[i] *= fac;
  return ap;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll k;
  cin >> n >> k;
  VI a(n);
  REP(i, 0, n) cin >> a[i];
  ModInt<> o7 = ModInt<>(3).pow((mod - 1) / 7);
  vector<ModInt<>> c(LEN);
  REP(i, 0, n) {
    int v = 0;
    int cur = 1;
    REP(j, 0, 7) {
      if (a[i] & 1 << j) {
        v += cur;
      }
      cur *= 7;
    }
    c[v] += 1;
  }
  fourier(c, o7);
  REP(i, 0, LEN) c[i] = c[i].pow(k);
  fourier(c, o7.inv());
  ModInt<> fac = ModInt<>(7).pow(mod - 8);
  REP(i, 0, LEN) c[i] *= fac;
  cout << c[0] << endl;
}
