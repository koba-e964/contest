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
typedef pair<ll, int> PLI;

const ll inf = 5e18;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL x(n), y(n);
  REP(i, 0, n) {
    ll p, q;
    cin >> p >> q;
    if (p > q) {
      x[i] = q;
      y[i] = p;
    } else {
      x[i] = p;
      y[i] = q;
    }
  }
  ll mi = inf;
  // Suppose min is taken by red, max is taken by blue.
  {
    ll red_mi = inf;
    ll red_ma = -inf;
    ll blue_mi = inf;
    ll blue_ma = -inf;
    REP(i, 0, n) {
      red_mi = min(red_mi, x[i]);
      red_ma = max(red_ma, x[i]);
      blue_mi = min(blue_mi, y[i]);
      blue_ma = max(blue_ma, y[i]);
    }
    mi = min(mi, (red_ma - red_mi) * (blue_ma - blue_mi));
  }
  // Suppose that min and max are both taken by red
  {
    PLI rmi(inf, -1);
    PLI rma(-inf, -1);
    REP(i, 0, n) {
      rmi = min(rmi, PLI(x[i], i));
      rma = max(rma, PLI(y[i], i));
    }
    ll bl = y[rmi.second];
    ll bh = x[rma.second];
    // if rmi.second == rma.second, we need to select n-1 elements, and bl, bh are ignored.
    if (bl > bh) {
      swap(bl, bh);
    }
    if (rmi.second == rma.second) {
      bl = inf;
      bh = -inf;
    }
    vector<PLI> pool;
    REP(i, 0, n) {
      if (i != rmi.second && i != rma.second) {
	pool.push_back(PLI(x[i], i));
	pool.push_back(PLI(y[i], i));
      }
    }
    sort(pool.begin(), pool.end());
    int kind = rmi.second == rma.second ? n - 1 : n - 2;
    if (kind > 0) {
      int pos = 0;
      VI tbl(n, 0);
      int cur = 0;
      ll bdiff = inf;
      REP(i, 0, pool.size()) {
	while (cur < kind && pos < (int) pool.size()) {
	  PLI c = pool[pos];
	  tbl[c.second] += 1;
	  if (tbl[c.second] == 1) {
	    cur += 1;
	  }
	  pos++;
	}
	if (cur == kind) {
	  ll diff = max(pool[pos - 1].first, bh)
	    - min(pool[i].first , bl);
	  
	  bdiff = min(bdiff, diff);
	}
	PLI c = pool[i];
	tbl[c.second] -= 1;
	if (tbl[c.second] == 0) {
	  cur -= 1;
	}
      }
      mi = min(mi, (rma.first - rmi.first) * bdiff);
    }
  }
  cout << mi << "\n";
}
