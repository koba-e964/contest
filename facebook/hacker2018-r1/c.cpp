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
const ll mod = 1e9 + 7;

const ll inf = 1e15;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    cout << "Case #" << case_nr << ":";
    int n, m;
    ll h1, h2, w, x, y, z;
    cin >> n >> m >> h1 >> h2 >> w >> x >> y >> z;
    VL a(m), b(m), u(m), d(m);
    REP(i, 0, m) {
      cin >> a[i] >> b[i] >> u[i] >> d[i];
      a[i]--, b[i]--;
    }
    VL h(n);
    h[0] = h1; h[1] = h2;
    REP(i, 2, n) {
      h[i] = (w * h[i - 2] + y) % z;
      h[i] = (h[i] + x * h[i - 1]) % z;
    }
    VL mi(n - 1, -inf), ma(n - 1, inf);
    REP(i, 0, m) {
      if (a[i] < b[i]) {
	REP(j, a[i], b[i]) {
	  ma[j] = min(ma[j], u[i]);
	  mi[j] = max(mi[j], -d[i]);
	}
      } else {
	REP(j, b[i], a[i]) {
	  ma[j] = min(ma[j], d[i]);
	  mi[j] = max(mi[j], -u[i]);
	}
      }
    }
    double hi = 2e6, lo = 0;
    REP(_, 0, 100) {
      bool ok = true;
      double mid = (hi + lo) / 2;
      double curlo = h[0] - mid, curhi = h[0] + mid;
      REP(i, 1, n) {
	double x = h[i] - mid, y = h[i] + mid;
	double z = curlo + mi[i - 1], w = curhi + ma[i - 1];
	x = max(x, z), y = min(y, w);
	if (0) {
	  DEBUGP(i);
	  DEBUGP(z);
	  DEBUGP(w);
	  DEBUGP(x);
	  DEBUGP(y);
	}
	curlo = x;
	curhi = y;
	if (x > y) {
	  ok = false;
	}
      }
      if (ok) {
	hi = mid;
      } else {
	lo = mid;
      }
    }
    cout << fixed << setprecision(17) << hi << "\n";
  }
}
