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
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 2000;
ll x[N], y[N];


struct cmp {
  bool operator()(pair<double, int> a, pair<double, int> b) {
    return a.first < b.first;
  }
};

inline
ll inn(
       ll ax,
       ll ay,
       ll bx,
       ll by) {
  return ax * bx + ay * by;
}
inline
ll out(
       ll ax,
       ll ay,
       ll bx,
       ll by) {
  return ax * by - ay * bx;
}
const int DEBUG = 0;

int main(void){
  int n;
  cin >> n;
  ll right = 0;
  ll obtuse = 0;
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
  }
  vector<pair<double, int> > t(2 * n - 2);
  vector<ll> xs(2 * n - 2);
  vector<ll> ys(2 * n - 2);
  REP(i, 0, n) {
    int p = 0;
    REP(j, 0, n) {
      if (i != j) {
	t[p].second = j;
	t[p].first = atan2(y[j] - y[i], x[j] - x[i]);
	p++;
      }
    }
    sort(t.begin(), t.begin() + n - 1, cmp());
    REP(j, 0, n - 1) { // doubling
      t[j + n - 1] = t[j];
    }
    REP(j, 0, 2 * n - 2) {
      xs[j] = x[t[j].second] - x[i];
      ys[j] = y[t[j].second] - y[i];
    }
    // shakutori
    int p0 = 0, p1 = 0, p2 = 0;
    ll mod_cnt = 0;
    REP(j, 0, n - 1) {
      p0 = max(p0, j);
      while (p0 < j + n - 1
	     && inn(xs[j], ys[j], xs[p0], ys[p0]) > 0
	     && out(xs[j], ys[j], xs[p0], ys[p0]) >= 0) { p0++; mod_cnt++; }
      p1 = max(p1, p0);
      while (p1 < j + n - 1
	     && inn(xs[j], ys[j], xs[p1], ys[p1]) >= 0
	     && out(xs[j], ys[j], xs[p1], ys[p1]) >= 0) { p1++; mod_cnt++; }
      p2 = max(p2, p1);
      while (p2 < j + n - 1 && (out(xs[j], ys[j], xs[p2], ys[p2]) > 0 || j == p2)) { p2++; mod_cnt++; }
      if (DEBUG) {
	printf("i = %d, j = %d, p0 = %d, p1 = %d, p2 = %d\n", i, j, p0, p1, p2);
      }
      right += p1 - p0;
      obtuse += p2 - p1;
      assert (mod_cnt < 6 * n);
    }
  }
  cout << ll(n) * (n - 1) * (n - 2) / 6 - right - obtuse << " ";
  cout << right << " ";
  cout << obtuse << endl;
}
