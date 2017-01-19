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

const double EPS = 1e-11;

/*
 * r1 > 0 ===> outer tangent
 * r1 < 0 ===> inner tangent
 * ax + by = c
 * Verified by: AOJ 2201 (http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=2162049)
 */
void circle_tangent(double x0, double y0, double r0,
		    double x1, double y1, double r1,
		    double &a, double &b, double &c) {
  double rd = r0 - r1;
  // p = ((x1 - x0) + (y1 - y0)i) / (u + (r0 - r1)i)
  double u = sqrt(pow(x1 - x0, 2) + pow(y1 - y0, 2) - rd * rd);
  double alpha = u * u + (r0 - r1) * (r0 - r1);
  double px = (x1 - x0) * u + (y1 - y0) * (r0 - r1);
  double py = - (x1 - x0) * (r0 - r1) + (y1 - y0) * u;
  px /= alpha;
  py /= alpha;
  a = py;
  b = -px;
  double qx = x0 - r0 * py;
  double qy = y0 + r0 * px;
  c = a * qx + b * qy;
}

int main(void){
  int n;
  while (cin >> n && n) {
    vector<double> x(n), y(n), r(n), m(n);
    REP(i, 0, n) {
      cin >> x[i] >> y[i] >> r[i] >> m[i];
    }
    int ma = 0;
    if (n >= 2) {
    REP(i, 0, n) {
      REP(j, 0, n) {
	int cnt = 0;
	double a, b, c;
#define ITSUMONO do {\
	cnt = 0;\
	REP(k, 0, n) {\
	  double dist = abs(a * x[k] + b * y[k] - c) / sqrt(a * a + b * b);\
	  if (dist > r[k] - EPS && dist < r[k] + m[k] + EPS) {\
	    cnt++;\
	  }\
	}\
	ma = max(ma, cnt);\
	}while(0)
	REP(s1, 0, 2) {
	  REP(s2, 0, 2) {
	    circle_tangent(x[i], y[i], r[i] + s1 * m[i],
			   x[j], y[j], r[j] + s2 * m[j],
		       a, b, c);
	    ITSUMONO;
	    circle_tangent(x[i], y[i], r[i] + s1 * m[i],
			   x[j], y[j], - (r[j] + s2 * m[j]),
		       a, b, c);
	    ITSUMONO;
	  }
	}
	
      }
    }
    } else {
      ma = 1;
    }
    cout << ma << endl;
  }
  
}
