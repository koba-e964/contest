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
const ll mod = 1e9 + 7;

double solve(double h, double w) {
  assert (h >= w);
  if (3 * h * h >= 4 * w * w) {
    return h * h / 4 + w * w;
  }
  // let A = h^2 - w^2
  // solve the equation (z^2 + A)^2 = 4h^2[(w - z)^2 + A]
  // the solution is in [0, w]
  double lo = 0.0;
  double hi = w;
  REP(loop_cnt, 0, 50) {
    double mid = (lo + hi) / 2;
    double a = h * h - w * w;
    double val = pow(mid * mid + a, 2) - 4 * h * h * (pow(w - mid, 2) + a);
    if (val > 0) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  return (w - lo) * (w - lo) + h * h;
}

int main(void){
  int t;
  cin >> t;
  while (t--) {
    double h, w;
    cin >> h >> w;
    if (h < w) {
      swap(h, w);
    }
    printf("%.15f\n", sqrt(solve(h, w)));
  }
}
