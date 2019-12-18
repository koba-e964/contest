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
typedef unsigned long long ull;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;

const int DEBUG = 1;

ll p;

ll powmod(ll a, ll e) {
  ll prod = 1;
  ll cur = a % p;
  while (e > 0) {
    if (e % 2) prod = prod * cur % p;
    cur = cur * cur % p;
    e /= 2;
  }
  return prod;
}

typedef vector<VL> mat;

VL factors(ll v);


// Find min {x | a^x = 1 (mod p) }.
ll period(ll a) {
  assert (a >= 1);
  assert (a < p);
  ll v = p - 1;
  VL fs = factors(v);
  for (ll i: fs) {
    while (v % i == 0) {
      if (powmod(a, v / i) == 1) {
        v /= i;
      } else {
        break;
      }
    }
  }
  return v;
}

const ll N = 80000;

ll det(const mat &a) {
  ll tmp = a[0][0] * a[1][1] % p;
  tmp -= a[0][1] * a[1][0] % p;
  return (tmp + p) % p;
}

mat mul(const mat &a, const mat &b) {
  mat c(2, VL(2));
  REP(i, 0, 2) REP(j, 0, 2) REP(k, 0, 2)
    c[i][k] = (c[i][k] + a[i][j] * b[j][k]) % p;
  return c;
}
mat powmat(const mat &a, ll e) {
  mat prod(2, VL(2));
  mat cur(a);
  REP(i, 0, 2) prod[i][i] = 1;
  while (e > 0) {
    if (e % 2) prod = mul(prod, cur);
    cur = mul(cur, cur);
    e /= 2;
  }
  return prod;
}

mat inv(const mat &a) {
  ll d = det(a);
  assert (d != 0);
  d = powmod(d, p - 2);
  mat b(2, VL(2));
  b[0][0] = a[1][1] * d % p;
  b[0][1] = a[0][1] * (p - d) % p;
  b[1][0] = a[1][0] * (p - d) % p;
  b[1][1] = a[0][0] * d % p;
  return b;
}

// Find min {x > 0 | a^x = b (mod p)}.
ll disc_log(ll a, ll b) {
  if (a <= 1) {
    return a == b ? 1 : -1;
  }
  ll per = period(a);
  map<ll, ll> big;
  REP(i, 0, N) {
    big[powmod(a, p - 1 - (N * (ll)i) % (p - 1)) * b % p] = N * (ll)i;
  }
  ll nxt = -1;
  REP(i, 0, N) {
    ll v = powmod(a, i);
    if (big.count(v)) {
      nxt = big[v] + i;
      break;
    }
  }
  return nxt < 0 ? -1 : (nxt + per - 1) % per + 1;
}

VL factors(ll v) {
  VL a;
  ll i = 2;
  while (i * i <= v) {
    int e = 0;
    while (v % i == 0) {
      e++;
      v /= i;
    }
    if (e) a.push_back(i);
    i++;
  }
  if (v > 1) a.push_back(v);
  return a;
}

ll mat_period(mat a) {
  assert (det(a) == 1);
  mat unit(2, VL(2));
  REP(i, 0, 2) unit[i][i] = 1;
  if (a == unit) return 1;
  if (mul(a, a) == unit) return 2;
  if (powmat(a, p) == unit) return p;
  if (powmat(a, 2 * p) == unit) return 2 * p;
  ll v = p * p - 1;
  assert (powmat(a, v) == unit);
  VL f1 = factors(p - 1), f2 = factors(p + 1);
  for (int f: f2) f1.push_back(f);
  for (int f: f1) {
    while (v % f == 0) {
      if (powmat(a, v / f) == unit) {
        v /= f;
      } else {
        break;
      }
    }
  }
  assert (powmat(a, v) == unit);
  return v;
}

ll disc_log_mat(mat a, mat b) {
  assert (det(a) == 1);
  ll per = mat_period(a);
  assert (per <= 2 * p + 2);
  map<mat, ll> big;
  mat step = powmat(inv(a), N);
  mat cur(b);
  REP(i, 0, N) {
    big[cur] = N * (ll) i;
    cur = mul(cur, step);
  }
  cur = mat(2, VL(2));
  REP(i, 0, 2) cur[i][i] = 1;
  ll nxt = -1;
  REP(i, 0, N) {
    if (big.count(cur)) {
      nxt = big[cur] + i;
      nxt %= per;
      break;
    }
    cur = mul(cur, a);
  }
  return nxt < 0 ? -1 : nxt;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  mat a(2, VL(2)), b(2, VL(2));
  cin >> p;
  REP(i, 0, 2) {
    REP(j, 0, 2) {
      cin >> a[i][j];
    }
  }
  REP(i, 0, 2) {
    REP(j, 0, 2) {
      cin >> b[i][j];
    }
  }
  // A = kI?
  if (a[0][1] == 0 && a[1][0] == 0 && a[0][0] == a[1][1]) {
    if (b[0][1] == 0 && b[1][0] == 0 && b[0][0] == b[1][1]) {
      cout << disc_log(a[0][0], b[0][0]) << endl;
    } else {
      cout << -1 << endl;
    }
    return 0;
  }
  if (DEBUG) {
    cerr << det(a) << endl;
    cerr << det(b) << endl;
  }
  // B = xA + yI
  VL cons;
  if (a[0][1] != 0) {
    cons.push_back(b[0][1] * powmod(a[0][1], p - 2) % p);
  }
  if (a[1][0] != 0) {
    cons.push_back(b[1][0] * powmod(a[1][0], p - 2) % p);
  }
  if (a[0][0] != a[1][1]) {
    ll diff = (a[0][0] - a[1][1] + p) % p;
    cons.push_back((b[0][0] - b[1][1] + p) % p * powmod(diff, p - 2) % p);
  }
  assert (cons.size() > 0);
  cons.erase(unique(cons.begin(), cons.end()), cons.end());
  if (cons.size() != 1) {
    cout << -1 << endl;
    return 0;
  }
  ll x = cons[0];
  ll y = (b[0][0] - x * a[0][0] % p + p) % p;
  if (DEBUG) {
    cerr << "B = " << x << " A + " << y << " I" << endl;
  }
  if (det(a) == 0) {
    // A^2 = z A. Find z = tr(A).
    ll z = (a[0][0] + a[1][1]) % p;
    if (y != 0) {
      cout << -1 << endl;
      return 0;
    }
    if (z == 0) {
      ll ans = x == 1 ? 1 : x == 0 ? 2 : -1;
      cout << ans << endl;
      return 0;
    }
    cout << disc_log(z, z * x % p) << endl;
    return 0;
  }
  assert (det(a) != 0);
  ll e = disc_log(det(a), det(b));
  if (e < 0) {
    cout << -1 << endl;
    return 0;
  }
  if (DEBUG) {
    cerr << "|A|^" << e << " = |B|" << endl;
  }
  ull per = period(det(a));
  mat ap = powmat(a, per);
  b = mul(b, powmat(inv(a), e));
  ull tmp = disc_log_mat(ap, b);
  if (DEBUG) {
    cerr << "tmp = " << tmp << endl;
    cerr << "per = " << per << endl;
  }
  if (tmp >= 0) {
    cout << tmp * per + (ull) e << endl;
  } else {
    cout << -1 << endl;
  }
}
