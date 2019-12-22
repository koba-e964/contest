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

const int N = 100010;
int n;
double x[N], r[N];

double lft[N], rgt[N];

double cb(double x) {
  return x * x * x;
}


void init() {
  lft[0] = r[0];
  REP(i, 1, n) {
    double arv = max(0.0, lft[i - 1] - (x[i] - x[i - 1]));
    lft[i] = cbrt(arv * arv * arv + r[i] * r[i] * r[i]);
  }
  rgt[n - 1] = r[n - 1];
  for (int i = n - 2; i >= 0; i--) {
    double arv = max(0.0, rgt[i + 1] - (x[i + 1] - x[i]));
    rgt[i] = cbrt(arv * arv * arv + r[i] * r[i] * r[i]);
  }
  if (DEBUG) {
    REP(i, 0, n) cerr << " " << lft[i]; cerr << endl;
    REP(i, 0, n) cerr << " " << rgt[i]; cerr << endl;
  }
}

double f(double c, int given_idx) {
  int idx = lower_bound(x, x + n, c) - x;
  if (DEBUG && given_idx != idx) {
    cerr << c << " " << given_idx << " != " << idx << endl;
    // assert (given_idx == idx);
  }
  double lft_r = idx > 0 ? max(0.0, lft[idx - 1] - (c - x[idx - 1])) : 0;
  double rgt_r = idx < n ? max(0.0, rgt[idx] - (x[idx] - c)) : 0;
  return cb(lft_r) + cb(rgt_r);
}

int main(void) {
  scanf("%d", &n);
  vector<pair<double, double> > xr;
  REP(i, 0, n) {
    double a, b;
    scanf("%lf%lf", &a, &b);
    xr.push_back(make_pair(a, b));
  }
  sort(xr.begin(), xr.end());
  REP(i, 0, n) {
    x[i] = xr[i].first;
    r[i] = xr[i].second;
  }
  init();
  double ma = 0.0;
  REP(i, 0, n - 1) {
    double lo = x[i], hi = x[i + 1];
    // (2/3)^90 ~= 1.4 * 10^-16
    REP(_, 0, 90) {
      double p = (2 * lo + hi) / 3, q = (lo + 2 * hi) / 3;
      double fp = f(p, i + 1), fq = f(q, i + 1);
      if (fp < fq) {
        lo = p;
      } else {
        hi = q;
      }
    }
    ma = max(ma, f((lo + hi) / 2, i + 1));
  }
  printf("%.15f\n", cbrt(ma));
}
