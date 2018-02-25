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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const double EPS = 1e-9;

int main(void) {
  ll r, x1, y1, x2, y2;
  cin >> r >> x1 >> y1 >> x2 >> y2;
  x2 -= x1;
  y2 -= y1;
  double xabs = sqrt(x2 * x2 + y2 * y2);
  if (xabs > r - EPS) {
    printf("%.15f %.15f %.15f\n", (double)x1, (double)y1, (double)r);
    return 0;
  }
  double ans_r = (r + xabs) / 2.0;
  double ans_x = xabs < EPS ? r / 2.0 : -x2 * (r - xabs) / (2 * xabs);
  double ans_y = xabs < EPS ? 0 : -y2 * (r - xabs) / (2 * xabs);
  printf("%.15f %.15f %.15f\n", (double)(x1 + ans_x), (double)(y1 + ans_y),
	 (double)ans_r);
}
