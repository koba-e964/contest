#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 100100;
int x[N], y[N];
ll res[N];

const ll inf = 1e16;

ll gcd(ll n, ll m) {
  if (m == 0) {
    return n;
  }
  return gcd(m, n % m);
}
typedef pair<ll, ll> PL;
PL ex_gcd(ll n, ll m) {
  if (m == 0) {
    return PL(1, 0);
  }
  ll q = n / m;
  PL res = ex_gcd(m, n % m);
  return PL(res.second, res.first - q * res.second);
}
// Two arithmetic progressions, intersection, meet (for search)
// t = n * lx = m * ly + b
ll solve2(ll n, ll m, ll b) {
  ll g = gcd(n, m);
  if (b % g != 0) {
    return inf;
  }
  PL ex = ex_gcd(n, m);
  // ex.first * n + ex.second * m == g
  ll lx = ex.first * b / g;
  ll ly = -ex.second * b / g;
  if (ly < 0) {
    ll q = (-ly) / (n / g) + 1;
    lx += q * m / g;
    ly += q * n / g;
  }
  if (ly >= n / g) {
    ll q = ly / (n / g);
    ly %= n / g;
    lx -= q * m / g;
  }
  //cout << "solve(" << n << "," << m << "," << b << ") = " << lx * n << endl;
  return lx * n;
}

ll solve(ll n, ll m, ll a, ll b) {
  n *= 2;
  m *= 2;
  // solve minimum t = n * lx + a = m * ly + b
  if (a > b) {
    swap(n, m);
    swap(a, b);
  }
  ll res = solve2(n, m, b - a);
  return res >= inf ? inf : res + a;
}

int main(void){
  int n, m, k;
  scanf("%d%d%d", &n, &m, &k);
  REP(i, 0, k) {
    scanf("%d%d", x + i, y + i);
  }
  ll en = 1e16;
  en = min(en, solve(n, m, 0, m));
  en = min(en, solve(n, m, n, 0));
  en = min(en, solve(n, m, n, m));

  REP(i, 0, k) {
    res[i] = en;
    res[i] = min(res[i], solve(n, m, x[i], y[i]));
    res[i] = min(res[i], solve(n, m, 2 * n - x[i], y[i]));
    res[i] = min(res[i], solve(n, m, x[i], 2 * m - y[i]));
    res[i] = min(res[i], solve(n, m, 2 * n - x[i], 2 * m - y[i]));
  }
  REP(i, 0, k) {
    printf("%lld\n", res[i] == en ? -1 : res[i]);
  }
}
