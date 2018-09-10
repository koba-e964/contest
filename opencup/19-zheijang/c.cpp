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

ll ext_gcd(ll a, ll b, ll &x, ll &y) {
  if (b == 0) {
    x = 1; y = 0; return a;
  }
  ll q = a / b;
  ll g = ext_gcd(b, a - q * b, x, y);
  ll z = x - q * y;
  x = y; y = z;
  return g;
}

ll invmod(ll a, ll m) {
  ll x, y;
  ext_gcd(a, m, x, y);
  x %= m;
  if (x < 0) x += m;
  return x;
}

const int N = 200100;
ll fac_m[N], fac_e[N];

void init(void) {
  int e = 0;
  fac_m[0] = 1;
  fac_e[0] = 0;
  int mi = 0;
  REP(i, 0, N) {
    int v = i + 1;
    while (v % 2 == 0) {
      e += 1;
      v /= 2;
      fac_e[i + 1] += 1;
    }
    if (i < N - 1) {
      fac_m[i + 1] = fac_m[i] * v;
      fac_e[i + 1] += fac_e[i];
    }
    mi = min(mi, (int) fac_e[i] - i - 1);
  }
}

ll shift(ll x, int e) {
  return x << e;
}

ll slow_mul(ll x, ll y, ll mod) {
  if ((mod & -mod) == mod) {
    return (x * y) & (mod - 1);
  }
  if (mod <= (1LL << 31) - 1) {
    return (x % mod) * (y % mod) % mod;
  }
  assert (0);
}

namespace math314 {
template<class T> T extgcd(T a, T b, T& x, T& y) { for (T u = y = 1, v = x = 0; a;) { T q = b / a; swap(x -= q * u, u); swap(y -= q * v, v); swap(b -= q * a, a); } return b; }
template<class T> T mod_inv(T a, T m) { T x, y; extgcd(a, m, x, y); return (m + x % m) % m; }
  //http://math314.hateblo.jp/entry/2015/05/07/014908
// ガーナーのアルゴリズム garnerのアルゴリズム
// x % m[i] == r[i] を満たす，最小の0以上の整数x について， x % mod を求める
// ex)
// x % 5  == 4
// x % 7  == 1
// x % 11 == 2
// x % 13 = ?
// -> x % (5*7*11) = 134 より， 最小のxは 134
// よって， x % 13 = 4 を返す
#define sz(x) (x).size()
  typedef pair<ll, ll> Pii;
  ll garner(vector<Pii> mr, ll mod) {
    mr.push_back(Pii(mod, 0));

    vector<ll> coffs(sz(mr), 1);
    vector<ll> constants(sz(mr), 0);
    REP(i, 0, sz(mr) - 1){
      // coffs[i] * v + constants[i] == mr[i].second (mod mr[i].first) を解く
      ll v = slow_mul(((mr[i].second - constants[i]) % mr[i].first + mr[i].first) % mr[i].first,
		      mod_inv<ll>(coffs[i], mr[i].first), mr[i].first);
      assert (v >= 0);

      for (int j = i + 1; j < (int) sz(mr); j++) {
	(constants[j] += slow_mul(coffs[j], v, mr[j].first)) %= mr[j].first;
	coffs[j] = slow_mul(coffs[j], mr[i].first, mr[j].first);
      }
    }

    return constants[sz(mr) - 1];
  }

template<ll mod, int primitive_root>
class NTT {
public:
  ll powmod(ll a, ll n) {
    ll ret = 1; ll p = a % mod; while (n) { if (n & 1) ret = ret * p % mod; p = p * p % mod; n >>= 1; }
    return ret;
  }
  ll get_mod() const { return mod; }
  void _ntt(vector<ll>& a, int sign) {
    const int n = sz(a);
    assert((n ^ (n&-n)) == 0); //n = 2^k

    const int g = primitive_root; //g is primitive root of mod
    ll h = powmod(g, (mod - 1) / n); // h^n = 1
    if (sign == -1) h = powmod(h, mod-2); //h = h^-1 % mod

    //bit reverse
    int i = 0;
    for (int j = 1; j < n - 1; ++j) {
      for (int k = n >> 1; k >(i ^= k); k >>= 1);
      if (j < i) swap(a[i], a[j]);
    }

    for (int m = 1; m < n; m *= 2) {
      const int m2 = 2 * m;
      const ll base = powmod(h, n / m2);
      ll w = 1;
      REP(x, 0, m) {
	for (int s = x; s < n; s += m2) {
	  ll u = a[s];
	  ll d = a[s + m] * w % mod;
	  a[s] = u + d;
	  if (a[s] >= mod) a[s] -= mod;
	  a[s + m] = u - d;
	  if (a[s + m] < 0) a[s + m] += mod;
	}
	w = w * base % mod;
      }
    }

    for (auto& x : a) if (x < 0) x += mod;
  }
  void ntt(vector<ll>& input) {
    _ntt(input, 1);
  }
  void intt(vector<ll>& input) {
    _ntt(input, -1);
    const ll n_inv = powmod((ll)sz(input), mod - 2);
    for (auto& x : input) x = x * n_inv % mod;
  }

	// 畳み込み演算を行う
  vector<ll> convolution(const vector<ll>& a, const vector<ll>& b){
    int ntt_size = 1;
    while (ntt_size < (int) sz(a) + (int) sz(b)) ntt_size *= 2;

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

//異なるmodのNTTを定義3つ定義
  typedef NTT<1012924417, 5> NTT_1;
  typedef NTT<1224736769, 3> NTT_2;
  typedef NTT<1007681537, 3> NTT_3;
  typedef NTT<1045430273, 3> NTT_4;

// 任意のmodで畳み込み演算 O(n log n)
// a.size() + b.size() < 2^23 を仮定している． 8 * 10^6 < 2^23 なので 競技プログラミングでは困らないはず
  vector<ll> int32mod_convolution(vector<ll> a, vector<ll> b,ll mod){
    for (auto& x : a) {
      x %= mod; // 最初に -mod < x < mod にしておく
      if (x < 0) x += mod;
    }
    for (auto& x : b) {
      x %= mod;
      if (x < 0)
	x += mod;
    }
    // 3種類のmodでNTTを行う
    // このmodは全て素数で，互いに異なる．
    NTT_1 ntt1; NTT_2 ntt2; NTT_3 ntt3; NTT_4 ntt4; // NTT_5 ntt5;
    auto x = ntt1.convolution(a, b); // x[i] = sum(a[i-j],b[j]) % ntt1.getmod();
    auto y = ntt2.convolution(a, b); // y[i] = sum(a[i-j],b[j]) % ntt2.getmod();
    auto z = ntt3.convolution(a, b); // z[i] = sum(a[i-j],b[j]) % ntt3.getmod();
    auto w = ntt4.convolution(a, b); // z[i] = sum(a[i-j],b[j]) % ntt3.getmod();
    // auto v = ntt5.convolution(a, b); // z[i] = sum(a[i-j],b[j]) % ntt3.getmod();

#define L(msg, x) if (0) { cerr << msg; REP(i, 0, (x).size()) cerr << " " << x[i]; cerr << endl; }
    L("x:", x);
    L("y:", y);
    L("z:", z);
    L("w:", w);
    //modを取らず畳み込みを行った場合，要素の最大値は 2^22 * (2^31 - 1)^2 となる．
    // ここで，2^22はmax(a.size(),b.size())の最大値
    // (2^31 - 1) は intの最大値，つまりmodの最大値．
    // 1224736769 * 469762049 * 167772161 > 2^22 * (2^31 - 1)^2 なので
    // CRT(中国人剰余定理)やgarnerのアルゴリズムで正しい値を復元出来る．
    // ただし，復元する際に，CRTではlong long超えてしまうため，garnerのアルゴリズムを用いる．
    vector<ll> ret(sz(x));
    vector<Pii> mr(4);
    REP(i, 0, sz(x)){
      mr[0].first = ntt1.get_mod(), mr[0].second = x[i];
      mr[1].first = ntt2.get_mod(), mr[1].second = y[i];
      mr[2].first = ntt3.get_mod(), mr[2].second = z[i];
      mr[3].first = ntt4.get_mod(), mr[3].second = w[i];
      // mr[4].first = ntt5.get_mod(), mr[4].second = v[i];
      // garnerのアルゴリズムで
      // t % ntt1.get_mod() = x[i],
      // t % ntt2.get_mod() = y[i],
      // t % ntt3.get_mod() = z[i],
      // を満たす，最小の0以上の整数 t について，
      // t % mod を求める．
      ret[i] = garner(mr, mod);
    }

    return ret;
  }
}

const ll lim = 1LL << 50;
using namespace math314;

#if 0

int main(void) {
  VL a(4), b(4);
  a[0] = 1e7;
  b[1] = 1e14;
  b[2] = 2e14;
  VL conv = int32mod_convolution(a, b, lim);
  REP(i, 0, conv.size()) {
    cerr << " " << conv[i];
  }
  cerr << endl;
}

#else

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n + 1), b(n + 1);
  REP(i, 0, n + 1) cin >> a[i];
  REP(i, 0, n + 1) cin >> b[i];
  // u[i] = a[i] / i! * 2^i mod 2^32
  // v[i] = b[i] / i! * 2^i mod 2^32
  vector<ll> u(n + 1), v(n + 1);
  ll tmp = 1;
  int e = 0;
  fac_m[0] = 1;
  fac_e[0] = 0;
  REP(i, 0, n + 1) {
    u[i] = shift(tmp * a[i], i - e) & (lim - 1);
    v[i] = shift(tmp * b[i], i - e) & (lim - 1);
    int v = i + 1;
    while (v % 2 == 0) {
      e += 1;
      v /= 2;
    }
    tmp *= invmod(v, lim);
    tmp %= lim;
  }
  VL conv = int32mod_convolution(u, v, lim);
  VL ans(n + 1);
  REP(i, 0, n + 1) {
    ll u = (conv[i] * fac_m[i]) >> (i - fac_e[i]);
    ans[i] = u & ((1LL << 32) - 1);
  }
  REP(i, 0, n + 1) {
    cout << ans[i] << (i == n ? "\n" : " ");
  }
}

#endif
