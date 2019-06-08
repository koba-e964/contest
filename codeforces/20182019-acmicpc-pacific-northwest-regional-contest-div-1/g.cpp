#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <cmath>
#include <vector>
#include <iomanip>
#include <cassert>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


ll mysqrt(ll x) {
  if (x <= 1) return x;
  ll pass = 1e7;
  ll fail = 0;
  while (pass - fail > 1) {
    ll mid = (pass + fail) / 2;
    if (mid * mid >= x) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  if (pass * pass - x < x - fail * fail) {
    return pass;
  }
  return fail;
}


double dist(double a, double b, double c, double d) {
  return mysqrt(1000000ll * ll((a - c) * (a - c) + (b - d) * (b - d))) / 1000.0;
}

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  double a, b, c, d, e, f;
  cin >> a >> b >> c >> d >> e >> f;
  double x[2] = {c, e};
  double y[2] = {d, f};
  double mi = dist(a, b, c, d);
  REP(i, 0, 2) REP(j, 0, 2) {
    mi = min(mi, dist(a, b, x[i], y[j]));
  }
  if (d <= b && b <= f) {
    mi = min(mi, min(abs(c - a), abs(e - a)));
  }
  if (c <= a && a <= e) {
    mi = min(mi, min(abs(d - b), abs(f - b)));
  }
  cout << fixed << setprecision(3) << mi << "\n";
}
