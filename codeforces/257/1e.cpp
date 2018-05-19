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
const ll mod = 1e9 + 7;
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
void add(ll &x, ll y) {
  x = (x + y) % mod;
}

int gcd(int x, int y) {
  while (y != 0) {
    int r = x % y;
    x = y; y = r;
  }
  return x;
}

const int N = 2100000;
ll sum[N];
ll sqsum[N];
ll qs[N], qs2[N], q0[N], q1[N];

ll r2[N], r1[N]; // sum((y^2-2y) * (y - 1)^k)

int pfac[N];

ll gcdsum[N];
ll gp0[N], gp1[N], gp2[N];

void init(void) {
  REP(i, 1, N) {
    sum[i] = sum[i - 1];
    add(sum[i], i);
  }
  REP(i, 1, N) {
    sqsum[i] = sqsum[i - 1];
    add(sqsum[i], (ll) i * (ll) i);
  }
  REP(i, 1, N) {
    q0[i] = q0[i - 1];
    q1[i] = q1[i - 1];
    qs[i] = qs[i - 1];
    qs2[i] = qs2[i - 1];
    ll tmp = (ll) i * (ll) (i - 2 + mod) % mod;
    add(q0[i], tmp);
    add(q1[i], tmp * (i - 1));
    add(qs[i], tmp * sum[i - 1]);
    add(qs2[i], tmp * sqsum[i - 1]);
  }
  REP(i, 1, N) {
    r2[i] = r2[i - 1];
    r1[i] = r1[i - 1];
    ll tmp = (ll) i * (ll) (i - 2 + mod) % mod;
    add(r1[i], tmp * (i - 1));
    ll u = tmp + 1;
    add(r2[i], tmp * u);
  }
  REP(i, 2, N) {
    if (pfac[i] != 0) continue;
    pfac[i] = i;
    REP(j, 2, (N - 1) / i + 1) pfac[i * j] = i;
  }
  REP(i, 1, N) {
    int v = i;
    ll prod = 1;
    while (v != 1) {
      int p = pfac[v];
      int e = 0;
      while (v % p == 0) {
	v /= p;
	e++;
      }
      ll tmp = powmod(p, e - 1);
      tmp = tmp * (e * (p - 1) + p) % mod;
      prod = tmp * prod;
    }
    gcdsum[i] = prod;
    gp0[i] = (gp0[i - 1] + prod) % mod;
    gp1[i] = (gp1[i - 1] + prod * i) % mod;
    gp2[i] = (gp2[i - 1] + (prod * i % mod) * i) % mod;
  }
}

ll solve(int n, int m) {
  ll tot = 0;
  int lim = min(n, m);
  ll tmp = qs2[lim];
  add(tot, mod - tmp);
  tmp = qs[lim];
  tmp = tmp * (m + n + 2) % mod;
  add(tot, tmp);
  tmp = q1[lim];
  tmp = tmp * (m + 1) % mod;
  tmp = tmp * (n + 1) % mod;
  add(tot, mod - tmp);
  ll coef = sqsum[lim];
  add(coef, (mod - m - n - 2) * sum[lim]);
  add(coef, (ll) ((ll) lim * (m + 1) % mod) * (n + 1) % mod);
  add(tot, coef * q0[lim]);
  tot = tot * 2 % mod;
  add(tot, mod - r2[lim]);
  add(tot, (m + n) * r1[lim]);
  add(tot, (q0[lim] * m % mod) * (mod - n));
  // I'm 100% sure the code above is correct!
  ll gcd_add = 0;
  add(gcd_add, (ll) (m + 1) * (n + 1) % mod * gp0[lim]);
  add(gcd_add, (m + n + 2) * (mod - gp1[lim]));
  add(gcd_add, gp2[lim]);
  add(tot, gcd_add * 2);
  return tot;
}

ll solve_naive(int n, int m) {
  ll tot = 0;
  REP(x, 1, min(n, m) + 1) {
    REP(y, 0, min(n, m) + 1 - x) {
      ll tmp = 0;
      add(tmp, (ll)x * (ll) (x - 2));
      add(tmp, (ll)y * (ll) (y - 2));
      add(tmp, 2 * gcd(x, y));
      tmp = tmp * (m - x - y + 1) % mod;
      tmp = tmp * (n - x - y + 1) % mod;
      add(tot, tmp);
    }
  }
  return tot;
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  while (t--) {
    int n, m;
    cin >> n >> m;
    ll ans = solve(n, m);
    cout << ans << "\n";
  }
}
