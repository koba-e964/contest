#include <algorithm>
#include <cmath>
#include <cstdio>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

const int RES = 2048;
const int T = 100 * RES + 1;
double memo_l[T];

/* 0.01 <= z <= 100 */
double lambert_W(double z, bool advised = true) {
  double lo = 0.0, hi = 100;
  int idx = z * RES;
  if (not advised) {
    if (log(z) >= 1) {
      lo = memo_l[idx - 1], hi = log(z);
    } else {
      lo = z / 2.719, hi = 1;
    }
  }
  if (advised) {
    lo = memo_l[idx], hi = memo_l[idx + 1];
  }
  REP(i, 0, 45) {
    if (hi - lo <= 5e-11) { break; }
    double mid = (lo + hi) / 2;
    if (mid * exp(mid) <= z) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  if (not advised) {
    memo_l[idx] = lo;
  }
  return lo;
}


double solve(int a, int b, double t) {
  if (b == 0) {
    return pow(t, 1.0 / a);
  }
  if (a == 0) {
    return exp(pow(t, 1.0 / b));
  }
  double res = lambert_W(pow(t, 1.0 / b) * a / b); 
  return exp(res * b / a);
}

int main(void){
  int m;
  scanf("%d", &m);
  REP(i, 0, T) {
    lambert_W(i / (double)RES, false);
  }
  REP(i, 0, m) {
    int a, b;
    double t;
    scanf("%d%d%lf", &a, &b, &t);
    printf("%.15f\n", solve(a, b, t));
  }
}
