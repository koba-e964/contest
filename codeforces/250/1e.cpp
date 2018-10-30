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
const ll gen = 3;
template<ll mod>
ll powmod(ll a, ll n) {
  ll ret = 1, p = a % mod;
  while (n) {
    if (n & 1) ret = ret * p % mod;
    p = p * p % mod;
    n >>= 1;
  }
  return ret;
}

template<ll mod, int primitive_root>
class NTT {
  ll powmod(ll a, ll n) {
    ll ret = 1, p = a % mod;
    while (n) {
      if (n & 1) ret = ret * p % mod;
      p = p * p % mod;
      n >>= 1;
    }
    return ret;
  }
public:
  ll get_mod() const { return mod; }
  void _ntt(vector<ll>& a, int sign) {
    const int n = a.size();
    assert((n ^ (n&-n)) == 0); //n = 2^k

    const int g = primitive_root; //g is primitive root of mod
    ll h = powmod(g, (mod - 1) / n); // h^n = 1
    if (sign == -1) h = powmod(h, mod-2); //h = h^-1 % mod

    //bit reverse
    int i = 0;
    for (int j = 1; j < n - 1; ++j) {
      for (int k = n >> 1; k > (i ^= k); k >>= 1);
      if (j < i) swap(a[i], a[j]);
    }

    for (int m = 1; m < n; m *= 2) {
      const int m2 = 2 * m;
      const ll base = powmod(h, n / m2);
      for (int r = 0; r < n; r += m2) {
        ll w = 1;
        for (int x = 0; x < m; ++x) {
          int s = x + r;
          ll u = a[s];
          ll d = a[s + m] * w % mod;
          a[s] = u + d;
          if (a[s] >= mod) a[s] -= mod;
          a[s + m] = u - d;
          if (a[s + m] < 0) a[s + m] += mod;
          w = w * base % mod;
        }
      }
    }

    for (auto& x : a) if (x < 0) x += mod;
  }
  void ntt(vector<ll>& input) {
    _ntt(input, 1);
  }
  void intt(vector<ll>& input) {
    _ntt(input, -1);
    const ll n_inv = powmod((ll)input.size(), mod - 2);
    for (auto& x : input) x = x * n_inv % mod;
  }

  // 畳み込み演算を行う
  vector<ll> convolution(const vector<ll>& a, const vector<ll>& b){
    int ntt_size = 1;
    while (ntt_size < (int) a.size() + (int) b.size()) ntt_size *= 2;

    vector<ll> _a = a, _b = b;
    _a.resize(ntt_size); _b.resize(ntt_size);
    for (auto &u: _a) {
      u %= mod;
    }
    for (auto &u: _b) {
      u %= mod;
    }
    ntt(_a);
    ntt(_b);

    REP(i, 0, ntt_size){
      (_a[i] *= _b[i]) %= mod;
    }

    intt(_a);
    return _a;
  }
};
NTT<mod, gen> ntt;

const int M = 1 << 17;
const int LOGM = 17;

// truncates the last len elements.
VL mul(const VL &a, const VL &b, int len) {
  VL aa(len), bb(len);
  REP(i, 0, len) {
    aa[i] = i >= (int) a.size() ? 0 : a[i];
    bb[i] = i >= (int) b.size() ? 0 : b[i];
  }
  VL res = ntt.convolution(aa, bb);
  res.resize(len);
  return res;
}

VL inv(const VL &a) {
  VL u(1);
  u[0] = powmod<mod>(a[0], mod - 2);
  REP(i, 0, LOGM) {
    VL tmp = mul(u, a, 1 << (i + 1));
    REP(j, 0, 1 << (i + 1)) {
      tmp[j] = tmp[j] == 0 ? 0 : mod - tmp[j];
    }
    tmp[0] = (tmp[0] + 2) % mod;
    u = mul(tmp, u, 1 << (i + 1));
  }
  return u;
}

// Assumes a[0] = 1. Returns (sqrt(a))^{-1} with constant term 1.
VL invsqrt(const VL &a) {
  VL u(1);
  u[0] = 1;
  REP(i, 0, LOGM) {
    VL tmp = mul(u, a, 1 << (i + 1));
    tmp = mul(tmp, u, 1 << (i + 1));
    REP(j, 0, 1 << (i + 1)) {
      tmp[j] = tmp[j] == 0 ? 0 : mod - tmp[j];
    }
    tmp[0] = (tmp[0] + 3) % mod;
    u = mul(tmp, u, 1 << (i + 1));
    ll fac = (mod + 1) / 2;
    REP(j, 0, 1 << (i + 1)) {
      u[j] = u[j] * fac % mod;
    }
  }
  return u;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VI c(n);
  REP(i, 0, n) cin >> c[i];
  VL t(M);
  REP(i, 0, n) t[c[i]] = 1;
  VL tmp(M);
  REP(i, 1, M) {
    tmp[i] = t[i] * (mod - 4) % mod;
  }
  tmp[0] = 1;
  VL invsqrttmp = invsqrt(tmp);
  VL num(invsqrttmp);
  REP(i, 0, M) num[i] = 2 * num[i] % mod;
  invsqrttmp[0] += 1; // Should be 2
  VL ans = inv(invsqrttmp);
  ans = mul(ans, num, M);
  REP(s, 1, m + 1) {
    cout << ans[s] << "\n";
  }
}
