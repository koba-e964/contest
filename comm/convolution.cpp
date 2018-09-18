//http://math314.hateblo.jp/entry/2015/05/07/014908 からコピー && 改造
template<class T> T ext_gcd(T a, T b, T& x, T& y) { for (T u = y = 1, v = x = 0; a;) { T q = b / a; swap(x -= q * u, u); swap(y -= q * v, v); swap(b -= q * a, a); } return b; }

template<class T> T zmod(T a, T b) {
  a %= b;
  if (a < 0) a += b;
  return a;
}

template<class T> T invmod(T a, T m) {
  ll x, y;
  ext_gcd(a, m, x, y);
  return zmod(x, m);
}

// ここを mod に応じて適切に変える
ll mulmod(ll x, ll y, ll mod) {
  return x * y % mod;
}


// garnerのアルゴリズム
// x % m[i] == r[i] を満たす，最小の0以上の整数x について， x % mod を求める
// ex)
// x % 5  == 4
// x % 7  == 1
// x % 11 == 2
// x % 13 = ?
// -> x % (5*7*11) = 134 より， 最小のxは 134
// よって， x % 13 = 4 を返す
typedef pair<ll, ll> Pii;
typedef pair<ll, ll> Pii;
ll garner(vector<Pii> mr, ll mod) {
  mr.push_back(Pii(mod, 0));

  vector<ll> coffs(mr.size(), 1);
  vector<ll> constants(mr.size(), 0);
  REP(i, 0, mr.size() - 1){
    // coffs[i] * v + constants[i] == mr[i].second (mod mr[i].first) を解く
    ll v = mulmod(zmod(mr[i].second - constants[i], mr[i].first),
                  invmod<ll>(coffs[i], mr[i].first), mr[i].first);
    assert (v >= 0);

    for (int j = i + 1; j < (int) mr.size(); j++) {
      (constants[j] += mulmod(coffs[j], v, mr[j].first)) %= mr[j].first;
      coffs[j] = mulmod(coffs[j], mr[i].first, mr[j].first);
    }
  }

  return constants[mr.size() - 1];
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

// Up to 2^20
typedef NTT<1012924417, 5> NTT_1;
typedef NTT<1224736769, 3> NTT_2;
typedef NTT<1007681537, 3> NTT_3;
typedef NTT<1045430273, 3> NTT_4;

vector<ll> arbmod_convolution(vector<ll> a, vector<ll> b, ll mod){
  for (auto& x : a)
    x = zmod(x, mod);
  for (auto& x : b)
    x = zmod(x, mod);
  NTT_1 ntt1; NTT_2 ntt2; NTT_3 ntt3; NTT_4 ntt4;;
  auto x = ntt1.convolution(a, b);
  auto y = ntt2.convolution(a, b);
  auto z = ntt3.convolution(a, b);
  auto w = ntt4.convolution(a, b);

  vector<ll> ret(x.size());
  vector<Pii> mr(4);
  REP(i, 0, x.size()){
    mr[0] = Pii(ntt1.get_mod(), x[i]);
    mr[1] = Pii(ntt2.get_mod(), y[i]);
    mr[2] = Pii(ntt3.get_mod(), z[i]);
    mr[3] = Pii(ntt4.get_mod(), w[i]);
    ret[i] = garner(mr, mod);
  }

  return ret;
}
