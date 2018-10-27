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
#include <bitset>

#define MOCK 0

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef unsigned long long int ull;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ull mod = 1012924417;
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

typedef NTT<mod, 5> NTTInst;

const int N = 1024;
const int NN = N * N;


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
#if MOCK
  n = 1000;
  k = 500;
#else
  cin >> n >> k;
#endif
  VI a(n);
#if MOCK
  REP(i, 0, n) a[i] = i + 1;
#else
  REP(i, 0, n) cin >> a[i];
#endif
  vector<ll> val(NN);
  for (unsigned int j = 0; j < (unsigned int) n; ++j) {
    val[a[j]] += 1;
  }
  NTTInst ntt;
  ntt.ntt(val);
  REP(i, 0, NN) val[i] = powmod(val[i], k);
  ntt.intt(val);
  VI ans;
  REP(i, 0, NN) {
    if (val[i]) {
      ans.push_back(i);
    }
  }
#if MOCK
  ull sum = 0;
  REP(i, 0, ans.size()) sum += ans[i];
  cout << sum << "\n";
#else
  REP(i, 0, ans.size()) {
    cout << ans[i] << (i == (int) ans.size() - 1 ? "\n" : " ");
  }
#endif
}
