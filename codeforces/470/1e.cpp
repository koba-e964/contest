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

template<class T>
void debug_vec(string name, const vector<T> &v) {
  cerr << name << ":";
  for (auto w: v) cerr << " " << w;
  cerr << endl;
}

typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 998244353;

const int N = 110000;
ll inv[N];


ll powmod(ll a, ll e) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

void init(void) {
  REP(i, 1, N) {
    inv[i] = powmod(i, mod - 2);
  }
}

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

const ll g = 3;

// v.size() must be the power of two
void fft(vector<ll> &a, int sgn) {
  int n = a.size();
  {
    int i = 0;
    for (int j = 1; j < n - 1; ++j) {
      for (int k = n >> 1; k > (i ^= k); k >>= 1) {}
      if (j < i) swap(a[i], a[j]);
    }
  }
  ll zeta = powmod(g, (mod - 1) / n);
  if (sgn == -1) zeta = powmod(zeta, mod - 2);
  for (int m = 1; m < n; m *= 2) {
    ll base = powmod(zeta, n / m / 2);
    ll cur = 1;
    for (int y = 0; y < m; ++y) {
      for (int x = 0; x < n; x += 2 * m) {
	int u = x + y;
	int v = x + y + m;
	ll s = a[u] + cur * a[v];
	ll t = a[u] + (mod - cur) * a[v];
	a[u] = s % mod, a[v] = t % mod;
      }
      cur = cur * base % mod;
    }
  }
}

VL convolution(const VL &a, const VL &b) {
  int p2 = 1;
  while ((int)a.size() + (int)b.size() - 1 > p2) p2 *= 2;
  VL a_(p2), b_(p2);
  REP(i, 0, a.size()) a_[i] = a[i];
  REP(i, 0, b.size()) b_[i] = b[i];
  fft(a_, 1);
  fft(b_, 1);
  REP(i, 0, p2) a_[i] = a_[i] * b_[i] % mod;
  fft(a_, -1);
  ll fac = powmod(p2, mod - 2);
  REP(i, 0, p2) a_[i] = fac * a_[i] % mod;
  if (0) {
    debug_vec("a", a);
    debug_vec("b", b);
    debug_vec("a*b", a_);
  }
  return a_;
}

VL divide_et_impera(const VL &v, int l, int r) {
  if (l > r) {
    return VL(1, 0);
  }
  if (l == r) {
    return VL(1, v[l]);
  }
  if (r - l <= 40) {
    // TODO slow
    VL s(0);
    for (int i = r; i >= l; --i) {
      int sz = s.size();
      VL t(sz + 1);
      REP(i, 0, sz) {
	add(t[i], s[i]);
	add(t[i + 1], (mod - 1) * s[i]);
      }
      add(t[0], v[i]);
      s = t;
    }
    return s;
  }
  int mid = (l + r + 1) / 2;
  VL fst = divide_et_impera(v, l, mid - 1);
  VL snd = divide_et_impera(v, mid, r);
  VL com(mid - l + 1);
  ll cur = 1;
  REP(i, 0, mid - l + 1) {
    com[i] = cur;
    cur = cur * (mid - l - i) % mod;
    cur = cur * inv[i + 1] % mod;
    cur = (mod - cur) % mod;
  }
  VL ans = convolution(com, snd);
  assert (fst.size() <= ans.size());
  REP(i, 0, fst.size()) {
    add(ans[i], fst[i]);
  }
  if (0) {
    debug_vec("v", v);
    DEBUGP(l);
    DEBUGP(r);
    debug_vec("fst", fst);
    debug_vec("snd", snd);
    debug_vec("com", com);
    debug_vec("ans", ans);
  }
  return ans;
}

VL change(const VL &v) {
  int n = v.size();
  return divide_et_impera(v, 0, n - 1);
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  init();
  int n;
  ll m;
  cin >> n >> m;
  VL p(n + 1);
  REP(i, 0, n + 1) cin >> p[i];
  VL t1 = change(p);
  REP(i, 0, n + 1) {
    ll tmp = powmod(powmod(i + 1, mod - 2), m);
    t1[i] = t1[i] * tmp % mod;
  }
  VL t2 = change(t1);
  REP(i, 0, n + 1) {
    cout << t2[i] << (i == n ? "\n" : " ");
  }
}
