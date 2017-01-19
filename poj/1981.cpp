#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <complex>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef complex<double> comp;



int main(void){
  int n;
  while (scanf("%d", &n), n) {
    vector<double> x(n), y(n);
    REP(i, 0, n) {
      double a, b;
      scanf("%lf%lf", &a, &b);
      x[i] = a;
      y[i] = b;
    }
    vector<pair<double, double> > cand;
    cand.reserve(n * n);
    REP(i, 0, n) {
      REP(j, i + 1, n) {
	double dist_sq = pow(x[i] - x[j], 2) + pow(y[i] - y[j], 2);
	if (dist_sq > 4) {
	  continue;
	}
	double coef = sqrt(1 / dist_sq - 1 / 4.0);
        double normal_x = (y[j] - y[i]) * coef;
        double normal_y = -(x[j] - x[i]) * coef;
	cand.push_back(make_pair((x[i] + x[j]) / 2 + normal_x, (y[i] + y[j]) / 2 + normal_y));
	cand.push_back(make_pair((x[i] + x[j]) / 2 - normal_x, (y[i] + y[j]) / 2 - normal_y));
      }
    }
    REP(i, 0, n) {
      cand.push_back(make_pair(x[i], y[i]));
    }
    int ma = 0;
    REP(i, 0, cand.size()) {
      pair<double, double> centre = cand[i];
      // cerr << centre.first << ", " << centre.second << endl;
      int cnt = 0;
      REP(j, 0, n) {
	cnt += pow(x[j] - centre.first, 2) + pow(y[j] - centre.second, 2) < 1.000002 ? 1 : 0;
      }
      ma = max(ma, cnt);
    }
    printf("%d\n", ma);
  }
}
