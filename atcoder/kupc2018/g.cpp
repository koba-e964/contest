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

void fail(void) {
  cout << "No" << endl;
  exit(0);
}

const ll lim = 1e16;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  VI c(n + 1);
  REP(i, 0, m) {
    int q;
    cin >> q;
    c[q] = 1;
  }
  VL a(n + 1);
  VL b(n + 1);
  VI pr(n + 1);
  REP(i, 2, n + 1) {
    if (pr[i] != 0) continue;
    pr[i] = i;
    for (int j = 2 * i; j <= n; j += i) {
      pr[j] = i;
    }
  }
  ll fac = 1e10;
  for (int i = 2; i <= n; ++i) {
    int cnt = 0;
    int up = -1;
    int v = i;
    while (v > 1) {
      int p = pr[v];
      while (v % p == 0) v /= p;
      cnt += 1;
      up = p;
    }
    if (cnt >= 2) {
      a[i] = 0;
    } else {
      a[i] = fac * log(up);
      if (c[i]) fail();
    }
  }
  REP(i, 1, n + 1) {
    for (int j = i; j <= n; j += i)
      b[j] += a[i];
  }
  REP(i, 1, n + 1) {
    if (a[i] > (ll) 1e12) fail();
  }
  REP(i, 1, n) if (b[i] >= b[i + 1]) fail();
  cout << "Yes" << endl;
  REP(i, 0, n) cout << a[i + 1] << (i == n - 1 ? "\n" : " ");
  // REP(i, 0, n) cerr << b[i + 1] << endl;
}
