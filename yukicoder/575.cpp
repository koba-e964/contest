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

const ll THRESHOLD = 100000;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n, m;
  cin >> n >> m;
  map<ll, int> fact;
  factorize(m, fact);
  ll mi = 1e16;
  for (auto v: fact) {
    ll p = v.first;
    int e = v.second;
    ll einn = 0;
    ll k = n;
    while (k > 0) {
      k /= p;
      einn += k;
    }
    mi = min(mi, einn / e);
  }
  double ret = 0;
  if (n < THRESHOLD) {
    REP(i, 1, n + 1) {
      ret += log10(i);
    }
  } else {
    // Stirling's approximation
    ret = n * (log10(n / exp(1)) + log10(n * sinh(1.0 / n) + 1.0 / (810.0 * pow(n, 6))) / 2.0);
    ret += log10(2 * acos(-1) * n) / 2.0;
  }
  ret -= mi * log10(m);
  ll tenexp = floor(ret);
  ret -= tenexp;
  ret = pow(10, ret);
  cout << ret << "e" << tenexp << "\n";
}
