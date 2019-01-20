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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const   double pi = acos(-1.0);


double sq(double x) { return x * x; }

typedef pair<double, double> PD;
vector<PD> lin(double a, double b, double c) {
  bool flip = abs(a) > abs(b);
  if (flip) swap(a, b);
  double base = a * c / (a * a + b * b);
  double filo = base * base - (c * c - b * b) / (a * a + b * b);
  filo = sqrt(filo);
  vector<PD> ans(2);
  ans[0].first = base + filo;
  ans[1].first = base - filo;
  REP(i, 0, 2) ans[i].second = (c - a * ans[i].first) / b;
  if (flip) {
    REP(i, 0, 2) swap(ans[i].first, ans[i].second);
  }
  return ans;
}
PD ray(double cx, double cy, double deg) {
  deg = deg + pi;
  double si = sin(deg);
  double co = cos(deg);
  double val = cx * si - cy * co;
  if (DEBUG) {
    cerr << " " << si << " x + " << -co << " y = " << val << endl;
  }
  vector<PD> ans = lin(si, -co, val);
  if (DEBUG && 1) {
    REP(i, 0, 2) {
      cerr << " " << ans[i].first << " " << ans[i].second << endl;
    }
  }
  double inn0 = co * (ans[0].first - cx) + si * (ans[0].second - cy);
  double inn1 = co * (ans[1].first - cx) + si * (ans[1].second - cy);
  if (DEBUG && 1) {
    DEBUGP(inn0);
    DEBUGP(inn1);
  }
  if (inn0 >= 1e-9) {
    if (DEBUG) cerr << "return 0" << endl;
    return ans[0];
  }
  if (DEBUG) cerr << "return 1" << endl;
  return ans[1];
}
double pos(PD x) {
  return atan2(x.second, x.first);
}
double range(PD x, PD y) {
  double a1 = pos(x);
  double a2 = pos(y);
  a2 -= a1;
  if (a2 < 0) a2 += 2 * pi;
  return a2 / 2 / pi;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  double x, y, r, sx, sy;
  int k;
  cin >> x >> y >> r >> sx >> sy >> k;
  double cx = sx, cy = sy;
  vector<PD> init(2);
  REP(i, 0, 2) {
    double phi = atan2(cy - y, cx - x);
    double dist = sqrt(sq(cx - x) + sq(cy - y));
    double theta = pi / 2 - (r >= dist ? 0 : acos(r / dist));
    // Rays to phi \pm theta
    double deg = phi + theta;
    init[0] = ray(cx, cy, deg);
    init[1] = ray(cx, cy, phi - theta);
  }
  double cur = range(init[0], init[1]);
  cout << setprecision(15) << cur << endl;
  vector<vector<PD> > hist(2);
  vector<double> tap(k);
  REP(dir, 0, 2) {
    double cx = init[dir].first;
    double cy = init[dir].second;
    hist[dir].push_back(init[dir]);
    REP(i, 0, k) {
      double phi = atan2(cy - y, cx - x);
      double dist = sqrt(sq(cx - x) + sq(cy - y));
      double theta = pi / 2 - (r >= dist ? 0 : acos(r / dist));
      double deg = phi + (dir == 0 ? theta : -theta);
      if (DEBUG) {
        DEBUGP(phi / pi);
        DEBUGP(theta / pi);
      }
      PD nxt = ray(cx, cy, deg);
      hist[dir].push_back(nxt);
      cx = nxt.first;
      cy = nxt.second;
      double u;
      if (dir == 0) {
        u = range(hist[dir][i + 1], hist[dir][i]);
      } else {
        u = range(hist[dir][i], hist[dir][i + 1]);
      }
      tap[i] += u;
    }
  }
  REP(i, 0, k) {
    cur += tap[i];
    if (cur >= 1) cur = 1;
    cout << setprecision(15) << cur << endl;
  }
  if (DEBUG) {
    REP(i, 0, 2) {
      REP(j, 0, hist[i].size()) {
        cerr << "(" << hist[i][j].first << "," << hist[i][j].second << ") ";
      }
      cerr << endl;
    }
  }
}
