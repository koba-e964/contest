#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <list>
#include <map>
#include <queue>
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
typedef pair<double, double> PD;

const double EPS = 1e-11;
double sq(double x) { return x * x; }

void solve(double ax, double ay, double bx, double by, int &ma,
		 const vector<double> &x, const vector<double> &y) {
  double sqdist = sq(ax - bx) + sq(ay - by);
  if (sqdist >= 4 + EPS) return;
  double dist = sqrt(sqdist);
  double ndist = sqrt(1 - sqdist / 4);
  double ex = (bx - ax) / dist, ey = (by - ay) / dist;
  int n = x.size();
  REP(i, 0, 2) {
    double coef = i == 0 ? 1 : -1;
    double cx = (ax + bx) / 2 + ey * coef * ndist;
    double cy = (ay + by) / 2 - ex * coef * ndist;
    int cnt = 0;
    REP(j, 0, n) {
      double sqdist = sq(x[j] - cx) + sq(y[j] - cy);
      if (sqdist <= 1 + EPS) {
	cnt += 1;
      }
    }
    ma = max(ma, cnt);
  }
}

int main(void) {
  int n;
  while(scanf("%d", &n) && n) {
    vector<double> x(n), y(n);
    REP(i, 0, n) scanf("%lf%lf", &x[i], &y[i]);
    int ma = 1;
    REP(i, 0, n) {
      REP(j, 0, i) {
        solve(x[i], y[i], x[j], y[j], ma, x, y);
      }
    }
    printf("%d\n", ma);
  }
}
