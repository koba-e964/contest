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


ll gcd(ll x, ll y) {
  while (y != 0) {
    ll r = x % y;
    x = y;;
    y = r;
  }
  return x;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll a, b;
  cin >> a >> b;
  ll g = gcd(a, b);
  a /= g, b /= g;
  map<ll, int> fa;
  factorize(a, fa);
  ll tot = 0;
  while (b != 0) {
    ll ma = 0;
    for (auto &pr: fa) {
      ll p = pr.first;
      if (a % p != 0) { continue; }
      ll cutup = b / p * p;
      ma = max(ma, cutup);
    }
    assert (ma < b);
    tot += b - ma;
    b = ma;
    ll g = gcd(a, b);
    a /= g;
    b /= g;
  }
  cout << tot << "\n";
}
