#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <complex>
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

typedef pair<ll, ll> P;

ll cross(const P& a, const P& b) {
  return a.first * b.second - a.second * b.first;
}
ll dot(const P& a, const P& b) {
  return a.first * b.first + a.second * b.second;
}

ll norm(P a) {
  return a.first * a.first + a.second * a.second;
}

void subt(P &a, P b) {
  a.first -= b.first;
  a.second -= b.second;
}

ll ccw(P a, P b, P c) {
  subt(b, a);
  subt(c, a);
  if (cross(b, c) > 0)   return +1;       // counter clockwise
  if (cross(b, c) < 0)   return -1;       // clockwise
  if (dot(b, c) < 0)     return +2;       // c--a--b on line
  if (norm(b) < norm(c)) return -2;       // a--b--c on line
  return 0;
}
typedef P point;
// http://www.prefield.com/algorithm/geometry/convex_hull.html
vector<point> convex_hull(vector<point> ps) {
  int n = ps.size(), k = 0;
  sort(ps.begin(), ps.end());
  vector<point> ch(2*n);
  for (int i = 0; i < n; ch[k++] = ps[i++]) // lower-hull
    while (k >= 2 && ccw(ch[k-2], ch[k-1], ps[i]) <= 0) --k;
  ch.resize(k);
  return ch;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  vector<P> acc(n + 1);
  REP(i, 0, n) {
    acc[i + 1] = P(i + 1, acc[i].second + a[i]);
  }
  auto t = convex_hull(acc);
  int m = t.size();
  vector<double> ans(n);
  REP(i, 0, m - 1) {
    int l = t[i].first;
    int r = t[i + 1].first;
    double avg = (double)(t[i + 1].second - t[i].second) / (double) (r - l);
    REP(i, l, r) ans[i] = avg;
  }
  REP(i, 0, n) {
    cout << setprecision(15) << ans[i] << "\n";
  }
}
