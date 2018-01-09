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
const ll mod = 1e9 + 7;

ll gcd(ll x, ll y) {
  return y == 0 ? x : gcd(y, x % y);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  ll g = 0;
  REP(i, 0, n) {
    cin >> a[i];
    g = gcd(g, a[i]);
  }
  if (g != 1) {
    cout << -1 << "\n";
    return 0;
  }
  int mi = n;
  REP(i, 0, n) {
    int cnt = 1;
    ll g = a[i];
    REP(j, i + 1, n + 1) {
      if (g == 1) { break; }
      if (j == n) break;
      g = gcd(g, a[j]);
      cnt++;
    }
    if (g == 1) {
      mi = min(mi, cnt);
    }
  }
  // in case we have multiple ones
  int one = 0;
  REP(i, 0, n) {
    if (a[i] == 1) one += 1;
  }
  if (one >= 1) {
    cout << n - one << "\n";
  } else {
    cout << mi + n - 2 << "\n";
  }
}
