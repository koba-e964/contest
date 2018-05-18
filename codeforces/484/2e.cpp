#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

ll gcd(ll a, ll b) {
  return b == 0 ? a : gcd(b, a % b);
}

ll ex_gcd(ll x, ll y, ll &a, ll &b) {
  if (y == 0) {
    a = 1;
    b = 0;
    return x;
  }
  ll q = x / y;
  ll r = x % y;
  ll res = ex_gcd(y, r, a, b);
  ll tmp = a - q * b;
  a = b;
  b = tmp;
  return res;
}


/*
 * Calculates the intersection of two arithmetic progressions,
 * x[n] = a + b * n and y[n] = c + d * n (n >= 0).
 * p1 = (a, b), p2 = (c, d)
 * Verified by: yukicoder No.261 (http://yukicoder.me/submissions/144768)
 */
pair<ll, ll> arith_prog_intersect(const pair<ll, ll> &p1, const pair<ll, ll> &p2) {
  if (p1.first > p2.first) {
    return arith_prog_intersect(p2, p1);
  }
  ll u, v;
  ll g = ex_gcd(p1.second, p2.second, u, v);
  ll lcm = p1.second / g * p2.second;
  if ((p1.first - p2.first) % g != 0) {
    return pair<ll, ll>(-1, -1);
  }
  ll diff = (p2.first - p1.first) / g;
  diff *= -v % (p1.second / g);
  diff %= p1.second / g;
  if (diff < 0) {
    diff += p1.second / g;
  }
  ll x = p2.first + diff * p2.second;
  return pair<ll, ll>(x, lcm);
}

int main(void) {
  ll n, m, x, y, vx, vy;
  cin >> n >> m >> x >> y >> vx >> vy;
  if (vx == 0) {
    if (x != 0 && x != n) {
      cout << -1 << endl;
    } else {
      cout << x << " " << (vy == -1 ? 0 : m) << endl;
    }
    return 0;
  }
  if (vy == 0) {
    if (y != 0 && y != m) {
      cout << -1 << endl;
    } else {
      cout << (vx == -1 ? 0 : n) << " " << y << endl;
    }
    return 0;
  }
  if (vx == -1) x = n - x;
  if (vy == -1) y = m - y;
  // arith_prog
  // (x + t, y + t) = (n * u, m * v)
  // t = n * u - x, so m * v = y - x + n * u (u >= 1, v >= 1)
  pair<ll, ll> res = arith_prog_intersect(make_pair(n + y - x, n), make_pair(m, m));
  if (res.first == -1) {
    cout << -1 << endl;
    return 0;
  }
  ll v = res.first / m;
  ll u = (m * v - y + x) / n;
  u %= 2;
  v %= 2;
  if (vx == -1) u = 1 - u;
  if (vy == -1) v = 1 - v;
  cout << n * u << " " << m * v << endl;
}
