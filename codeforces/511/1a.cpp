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

#define MOCK 0

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int gcd(int x, int y) {
  while (y != 0) {
    int r = x % y;
    x = y; y = r;
  }
  return x;
}

const int W = 15001000;
int fac[W];
void init(void) {
  REP(i, 2, W) {
    if (fac[i] != 0) continue;
    fac[i] = i;
    for (int j = 2 * i; j < W; j += i) {
      fac[j] = i;
    }
  }
}

const int inf = 1e8;

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
#if MOCK
  n = 300000;
#else
  cin >> n;
#endif
  VI a(n);
#if MOCK
  REP(i, 0, n) a[i] = W - 1000 - i;
#else
  REP(i, 0, n) cin >> a[i];
#endif
  int g = 0;
  REP(i, 0, n) g = gcd(g, a[i]);
  REP(i, 0, n) a[i] /= g;
  map<int, int> occur;
  REP(i, 0, n) {
    int v = a[i];
    while (v != 1) {
      int p = fac[v];
      occur[p] += 1;
      while (v % p == 0) {
        v /= p;
      }
    }
  }
  int mi = inf;
  for (PI e: occur) {
    mi = min(mi, n - e.second);
  }
  if (mi >= inf) {
    cout << -1 << endl;
  } else {
    cout << mi << endl;
  }
}
