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

#define MOCK 0

ll gcd(ll x, ll y) {
  while (y != 0) {
    ll r = x % y;
    x = y;
    y = r;
  }
  return x;
}

const int N = 710;
bool dpl[N][N];
bool dpr[N][N];

const int K = 10000;
bool pr[K];
VI primes;
void init(void) {
  REP(i, 2, K) pr[i] = i;
  REP(i, 2, K) {
    if (pr[i]) {
      for (int j = 2 * i; j < K; j += i) pr[j] = 0;
    }
  }
  REP(i, 2, K) if (pr[i]) primes.push_back(i);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
#if MOCK
  init();
  REP(i, 0, n) a[i] = primes[i];
#else
  REP(i, 0, n) cin >> a[i];
#endif
  vector<VI> g(n, VI(n));
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (i != j) {
	g[i][j] = gcd(a[i], a[j]) != 1;
      }
    }
  }
  REP(i, 0, n) {
    dpl[i][i] = 1;
    dpr[i][i] = 1;
  }
  REP(s, 1, n) {
    REP(i, 0, n) {
      if (i + s < n) {
	bool ok = false;
	REP(j, i + 1, i + s + 1) {
	  if (g[i][j] && dpl[j][i + 1] && dpr[j][i + s]) {
	    ok = true;
	    break;
	  }
	}
	dpr[i][i + s] |= ok;
      }
      if (i - s >= 0) {
	bool ok = false;
	REP(j, i - s, i) {
	  if (g[i][j] && dpr[j][i - 1] && dpl[j][i - s]) {
	    ok = true;
	    break;
	  }
	}
	dpl[i][i - s] |= ok;
      }
    }
  }
  bool ok = false;
  REP(i, 0, n) {
    if (dpl[i][0] && dpr[i][n - 1]) {
      ok = true;
    }
  }
  cout << (ok ? "Yes" : "No") << endl;
}
