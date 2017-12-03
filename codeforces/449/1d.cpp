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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
/**
 * For every prime factor p, perform result[p] += e. (where n = \prod p^e)
 * Verified by: Codeforces #400 E
 *              (http://codeforces.com/contest/776/submission/24956471)
 */
void factorize(long long v, std::map<long long, int> &result) {
  long long p = 2;
  while (v > 1 && p * p <= v) {
    int cnt = 0;
    while (v % p == 0) {
      cnt++;
      v /= p;
    }
    if (cnt > 0) {
      if (result.count(p) == 0) {
	result[p] = 0;
      }
      result[p] += cnt;
    }
    p += p == 2 ? 1 : 2;
  }
  if (v > 1) {
    if (result.count(v) == 0) {
      result[v] = 0;
    }
    result[v] += 1;
  }
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

const int N = 123456;
int n;
ll p;
VL q;
VI e;
ll fact[N];
ll invfact[N];
VI fact_e[N];

void init(void) {
  map<ll, int> factor;
  factorize(p, factor);
  for (map<ll, int>::iterator it = factor.begin(); it != factor.end(); ++it) {
    q.push_back(it->first);
    e.push_back(it->second);
  }
  int l=e.size();
  fact_e[0] = VI(l, 0);
  fact[0] = invfact[0] = 1;
  REP(i, 1, N) {
    int v = i;
    fact_e[i] = fact_e[i - 1];
    REP(j, 0, l) {
      while (v % q[j] == 0) {
	fact_e[i][j] += 1;
	v /= q[j];
      }
    }
    fact[i] = fact[i - 1] * v % p;
    ll dummy2;
    ll dummy = ex_gcd(fact[i], p, invfact[i], dummy2);
    assert (dummy == 1);
    invfact[i] = (invfact[i] % p + p) % p;
  }
}




ll tri_comb(int x, int y) {
  ll t = 1;
  int len = e.size();
  t = (fact[n] * invfact[x]) % p;
  t = t * invfact[y] % p;
  t = t * invfact[n - x - y] % p;
  VI es(len);
  REP(i, 0, len) {
    es[i] = fact_e[n][i] - fact_e[x][i] - fact_e[y][i] - fact_e[n - x - y][i];
    REP(j, 0, es[i]) {
      t = t * q[i] % p;
    }
  }
  return t;
}


ll calc(int l) {
  ll tot = 0;
  REP(i, 0, n - l + 1) {
    if ((n + i + l) % 2 == 0) {
      tot = (tot + tri_comb(i, (n - i - l) / 2)) % p;
    }
  }
  return tot;
}

// Reference: https://twitter.com/DEGwer3456/status/936994487309557760
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int l, r;
  cin >> n >> p >> l >> r;
  init();
  cout << (calc(l) + calc(l + 1) - calc(r + 1) - calc(r + 2) + 2 * p) % p
       << "\n";
}
