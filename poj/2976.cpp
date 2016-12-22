#include <algorithm>
#include <cmath>
#include <cstdio>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;



int main(void){
  int n, k;
  while (scanf("%d%d", &n, &k) && (n != 0 || k != 0)) {
    vector<double> a(n), b(n);
    REP(i, 0, n) {
      scanf("%lf", &a[i]);
    }
    REP(i, 0, n) {
      scanf("%lf", &b[i]);
    }
    double lo = 0;
    double hi = 1;
    REP(loop_cnt, 0, 20) {
      double mid = (lo + hi) / 2;
      double tot = 0;
      vector<double> neg;
      REP(i, 0, n) {
	double diff = a[i] - b[i] * mid;
	if (diff < 0) {
	  neg.push_back(diff);
	} else {
	  tot += diff;
	}
      }
      sort(neg.rbegin(), neg.rend());
      REP(i, 0, max(0, (int)neg.size() - k)) {
	tot += neg[i];
      }
      if (tot >= 0) {
	lo = mid;
      } else {
	hi = mid;
      }
    }
    printf("%d\n", (int) floor(lo * 100 + 0.5));
  }
  
}
