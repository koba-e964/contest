#include <cassert>
#include <cmath>
#include <cstdio>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

const double EPS = 3e-13;

/*
 * Verified by: POJ 1418 (http://poj.org/problem?id=1418)
 */
void circle_intersect(double r1,
		      double x2, double y2, double r2,
		      bool &ok,
		      pair<double, double> &p1,
		      pair<double, double> &p2
		      ) {
  // Intersection of
  // x^2 + y^2 = r1^2 and 2 * x2 * x + 2 * y2 * y = r1^2 - r2^2 + x2^2 + y2^2
  double a = 2 * x2;
  double b = 2 * y2;
  double c = r1 * r1 - r2 * r2 + x2 * x2 + y2 * y2;
  double dist = c / sqrt(a * a + b * b);
  if (dist >= r1 + EPS) {
    ok = false;
    return;
  }
  double tt = sqrt(r1 * r1 - dist * dist);
  double r = sqrt(x2 * x2 + y2 * y2);
  double ux = x2 / r;
  double uy = y2 / r;
  p1.first = ux * dist + uy * tt;
  p1.second = uy * dist - ux * tt;
  p2.first = ux * dist - uy * tt;
  p2.second = uy * dist + ux * tt;
  ok = true;
}

void circle_line_intersect(double r, double x0, double y0,
			   double a, double b, double c,
			   bool &intersect,
			   pair<double, double> &p1,
			   pair<double, double> &p2
			   ) {
  /*
   * (x - x0)^2 + (y - y0)^2 = r^2
   * a * x + b * y = c
   */
  double rab = sqrt(a * a + b * b);
  double dist = (a * x0 + b * y0 - c) / rab;
  if (abs(dist) >= r + EPS) {
    intersect = false;
    return;
  }
  double x1 = x0 - dist * a / rab;
  double y1 = y0 - dist * b / rab;
  double tt = sqrt(r * r - dist * dist);
  intersect = true;
  p1.first = x1 + tt * b / rab;
  p1.second = y1 - tt * a / rab;
  p2.first = x1 - tt * b / rab;
  p2.second = y1 + tt * a / rab;
}

/*
 * Verified by: POJ 1418 (http://poj.org/problem?id=1418)
 */
void line_points(pair<double, double> p1,
		 pair<double, double> p2,
		 double &a, double &b, double &c) {
  a = p2.second - p1.second;
  b = p1.first - p2.first;
  c = a * p1.first + b * p1.second;
}


typedef pair<double, double> PD;


/*
 * Reference: http://inazz.jp/pku/1418/
 */
int main(void){
  int n;
  while (scanf("%d", &n) && n) {
    vector<double> x(n), y(n), r(n);
    REP(i, 0, n) {
      scanf("%lf%lf%lf", &x[i], &y[i], &r[i]);
    }
    int cnt = 0;
    const double pi = acos(-1.0);
    vector<PD> occur;
    REP(j, 0, n) {
      REP(k, j + 1, n) {
	bool intersect;
	PD p1, p2;
	circle_intersect(r[j], x[k] - x[j], y[k] - y[j], r[k], intersect,
			 p1, p2);
	if (not intersect) {
	  continue;
	}
	p1.first += x[j];
	p1.second += y[j];
	p2.first += x[j];
	p2.second += y[j];
	occur.push_back(p1);
	occur.push_back(p2);
      }
    }
    REP(i, 0, n) {
      occur.push_back(PD(x[i], y[i]));
      occur.push_back(PD(x[i] + r[i], y[i]));
    }
    set<int> tops;
    REP(i, 0, occur.size()) {
      PD p = occur[i];
      int t = -1;
      REP(j, 0, n) {
	if (pow(x[j] - p.first, 2) + pow(y[j] - p.second, 2)
	    <= r[j] * (r[j] - EPS)) {
	  t = j;
	}
      }
      if (t >= 0) {
	tops.insert(t);
      }
      // t is the top circle which strictly contains p.
      int cnt = 0;
      REP(j, t + 1, n) {
	if (abs(pow(x[j] - p.first, 2) + pow(y[j] - p.second, 2)
		- r[j] * r[j]) <= EPS * r[j]) {
	  cnt++;
	  tops.insert(j);
	}
      }
      assert (cnt <= 2);
    }
    printf("%lu\n", tops.size());
  }
}
