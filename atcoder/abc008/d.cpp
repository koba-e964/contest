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
const ll mod = 1e9 + 7;

const int DEBUG = 0;

const int H = 81;
int dp[H][H][H][H] = {};

int main(void){
  int w, h, n;
  cin >> w >> h >> n;
  VI x(n), y(n);
  // Coordinate compression. Very tedious...
  set<int> xs, ys;
  map<int, int> xs_inv, ys_inv;
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
    x[i]--, y[i]--;
    xs.insert(x[i]);
    ys.insert(y[i]);
    xs.insert(x[i] + 1);
    ys.insert(y[i] + 1);
  }
  xs.insert(0);
  ys.insert(0);
  xs.insert(w);
  ys.insert(h);
  VI xs_v(xs.begin(), xs.end());
  VI ys_v(ys.begin(), ys.end());
  REP(i, 0, xs_v.size()) {
    xs_inv[xs_v[i]] = i;
  }
  REP(i, 0, ys_v.size()) {
    ys_inv[ys_v[i]] = i;
  }
  REP(i, 0, n) {
    x[i] = xs_inv[x[i]];
    y[i] = ys_inv[y[i]];
  }
  w = xs_inv[w];
  h = ys_inv[h];
  assert (h <= 80 && w <= 80);
  REP(i, 0, w) {
    REP(k, 0, h) {
      REP(j, 0, w - i) {
	REP(l, 0, h - k) {
	  int xe = i + j + 1;
	  int ye = k + l + 1;
	  int &ret = dp[j][xe][l][ye];
	  int ma = 0;
	  int width = xs_v[xe] - xs_v[j] + ys_v[ye] - ys_v[l] - 1;
	  REP(p, 0, n) {
	    if (j <= x[p] && x[p] < xe && l <= y[p] && y[p] < ye) {
	      int tmp = 0;
	      tmp += dp[j][x[p]][l][y[p]];
	      tmp += dp[j][x[p]][y[p] + 1][ye];
	      tmp += dp[x[p] + 1][xe][l][y[p]];
	      tmp += dp[x[p] + 1][xe][y[p] + 1][ye];
	      ma = max(ma, tmp + width);
	    }
	  }
	  if (DEBUG && ma) {
	    cerr << "dp[" << j << "," << xe << "," << l << "," << ye <<
	      "]=" << ma << endl;
	  }
	  ret = ma;
	}
      }
    }
  }
  cout << dp[0][w][0][h] << endl;
}
