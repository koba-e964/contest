#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

#define NAIVE 0

const int DEBUG = 0;

ll add64(ll x, ll y, ll m) {
  ll res = x + y;
  if (res >= m) res -= m;
  return res;
}

ll mul64(ll x, ll y, ll m) {
  ll sum = 0;
  ll cur = x;
  while (y > 0) {
    if (y % 2) {
      sum = add64(sum, cur, m);
    }
    cur = add64(cur, cur, m);
    y /= 2;
  }
  return sum;
}

ll mod;

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

const int M = 22;
ll comb[M][M];

void init(void) {
  comb[0][0] = 1;
  REP(i, 1, M) {
    REP(j, 0, M) {
      if (j > 0) {
	add(comb[i][j], comb[i - 1][j - 1]);
      }
      add(comb[i][j], comb[i - 1][j]);
    }
  }
}

ll mat[M][M][M];

VL mul(const VL &a, const VL &b) {
  int n = a.size();
  VL ret(n);
  REP(i, 0, n) {
    REP(j, 0, n) {
      REP(k, 0, n) {
	add(ret[k], (mat[i][j][k] * a[i] % mod) * b[j]);
      }
    }
  }
  return ret;
}

VL conv_fft_sub(const VL &a, const VL &b, ll mo) {
  if (a.size() <= 1) {
    return VL(1, mul64(a[0], b[0], mo));
  }
  int len = a.size();
  VL ap(len / 2), am(len / 2);
  VL bp(len / 2), bm(len / 2);
  REP(i, 0, len / 2) {
    ap[i] = add64(a[i], a[i + len / 2], mo);
    am[i] = add64(a[i], - a[i + len / 2] + mo, mo);
    bp[i] = add64(b[i], b[i + len / 2], mo);
    bm[i] = add64(b[i], - b[i + len / 2] + mo, mo);
  }
  VL x = conv_fft_sub(ap, bp, mo);
  VL y = conv_fft_sub(am, bm, mo);
  VL ret(len);
  REP(i, 0, len / 2) {
    ret[i] = (x[i] + y[i]) / 2;
    ret[i] %= mo;
    ret[i + len / 2] = add64(ret[i], -y[i] + mo, mo);
  }
  REP(i, 0, len) {
    assert (ret[i] <= mo);
  }
  return ret;
}


VL conv_fft(const VL &a, const VL &b) {
  return conv_fft_sub(a, b, mod * (ll) a.size());
}

VL conv_naive(const VL &a, const VL &b) {
  VL ans(a.size());
  REP(i, 0, a.size()) {
    REP(j, 0, a.size()) {
      add(ans[i ^ j], a[i] * b[j]);
    }
  }
  return ans;
}

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int m;
  ll t;
  cin >> m >> t >> mod;
  ll oldt = t;
  init();
  VL e0(1 << m);
  REP(i, 0, 1 << m) {
    cin >> e0[i];
    e0[i] %= mod;
  }
  VL b(m + 1);
  REP(i, 0, m + 1) cin >> b[i];
  VL vec(m + 1);
  REP(i, 0, m + 1) vec[i] = b[i] % mod;
  // mat
  REP(i, 0, m + 1) {
    REP(j, 0, m + 1) {
      REP(x, 0, min(i, j) + 1) {
	int y = i + j - 2 * x;
	if (y > m) continue;
	ll tmp = comb[y][i - x];
	tmp = tmp * comb[m - y][x] % mod;
	mat[i][j][y] = tmp;
      }
    }
  }
  if (DEBUG) {
    REP(i, 0, m + 1) {
      REP(j, 0, m + 1) {
	cerr << "|" << i << ">|" << j <<">=";
	REP(x, 0, m + 1) {
	  cerr << " + " << mat[i][j][x] << "|" << x << ">";
	}
	cerr << endl;
      }
    }
  }
  VL cur(vec), prod(m + 1);
  prod[0] = 1;
  while (t > 0) {
    if (t % 2 == 1) {
      prod = mul(prod, cur);
    }
    cur = mul(cur, cur);
    t /= 2;
  }
  if (DEBUG) {
    cerr << "init^" << oldt << "=" << endl;
    REP(i, 0, m + 1) {
      cerr << " + " << prod[i] << "|" << i << ">";
    }
    cerr << endl;
  }
  // convolution
  VL coef(1 << m);
  REP(i, 0, 1 << m) {
    coef[i] = prod[__builtin_popcount(i)];
  }
  VL ans = conv_fft(e0, coef);
#if NAIVE
  VL ans_naive = conv_naive(e0, coef);
#endif
  if (DEBUG) {
    cerr << "ans:";
    REP(i, 0, 1 << m) {
      cerr << " " << ans[i];
    }
    cerr << endl;
#if NAIVE
    cerr << "naive:";
    REP(i, 0, 1 << m) {
      cerr << " " << ans_naive[i];
    }
    cerr << endl;
#endif
  }
  REP(i, 0, 1 << m) cout << ans[i] % mod << endl;
}
