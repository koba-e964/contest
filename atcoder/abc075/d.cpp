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

const int N = 52;
int bit[N][N];

ll accum(int x, int y) {
  ll tot = 0;
  while (x > 0) {
    int yc = y;
    while (yc > 0) {
      tot += bit[x][yc];
      yc &= yc - 1;
    }
    x &= x - 1;
  }
  return tot;
}

void update(int x, int y, ll val) {
  if (x <= 0 || y <= 0) {
    return;
  }
  while (x < N) {
    int yc = y;
    while (yc < N) {
      bit[x][yc] += val;
      yc += yc & (-yc);
    }
    x += x & (-x);
  }
}

map<int, int> comp(const VI &x, VI &inv) {
  VI y(x);
  sort(y.begin(), y.end());
  map<int, int> ret;
  REP(i, 0, y.size()) {
    ret[y[i]] = i;
  }
  inv = y;
  return ret;
}

int main(void) {
  int n, k;
  cin >> n >> k;
  VI x(n), y(n);
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
  }
  VI xc_inv, yc_inv;
  map<int, int> xc = comp(x, xc_inv), yc = comp(y, yc_inv);
  REP(i, 0, n) {
    update(xc[x[i]] + 1, yc[y[i]] + 1, 1);
  }
  ll mi = 5e18;
  REP(i, 1, n + 1) {
    REP(j, i, n + 1) {
      REP(a, 1, n + 1) {
	REP(b, a, n + 1) {
	  if (accum(j, b) - accum(j, a - 1) - accum(i - 1, b) + accum(i - 1, a - 1) >= k) {
	    mi = min(mi, ll(xc_inv[j - 1] - xc_inv[i - 1]) *
		     ll(yc_inv[b - 1] - yc_inv[a - 1]));
	  }
	}
      }
    }
  }
  cout << mi << endl;
}
