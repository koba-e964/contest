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

ll gcd(ll x, ll y) {
  while (y != 0) {
    ll r = x % y;
    x = y;
    y = r;
  }
  return x;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n), b(n);
  REP(i, 0, n) cin >> a[i] >> b[i];
  REP(i, 0, n) a[i] = a[i] / gcd(a[i], b[i]) * b[i];
  ll g = 0;
  REP(i, 0, n) g = gcd(a[i], g);
  if (g == 1) {
    cout << -1 << "\n";
    return 0;
  }
  for (ll x = 2; x <= 50000; ++x) {
    if (g % x == 0) {
      cout << x << "\n";
      return 0;
    }
  }
  // not found... trying to factor by hand
  REP(i, 0, n) {
    ll tmp = gcd(g, a[i]);
    if (tmp != 1 && tmp != g) {
      cout << tmp << "\n";
      return 0;
    }
    tmp = gcd(g, b[i]);
    if (tmp != 1 && tmp != g) {
      cout << tmp << "\n";
      return 0;
    }
  }
  // g is prime.
  cout << g << endl;
}
