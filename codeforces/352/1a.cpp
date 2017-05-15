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

inline double sq(double x) {
  return x * x;
}

int main(void){
  double ax, ay, bx, by, tx, ty;
  cin >> ax >> ay >> bx >> by >> tx >> ty;
  int n;
  cin >> n;
  ax -= tx;
  ay -= ty;
  bx -= tx;
  by -= ty;
  vector<double> x(n), y(n);
  vector<double> d(n);
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
    x[i] -= tx;
    y[i] -= ty;
    d[i] = sqrt(sq(x[i]) + sq(y[i]));
  }
  vector<pair<double, int> > ad(n), bd(n);
  REP(i, 0, n) {
    double at = sqrt(sq(ax - x[i]) + sq(ay - y[i]));
    at -= d[i];
    double bt = sqrt(sq(bx - x[i]) + sq(by - y[i]));
    bt -= d[i];
    ad[i] = make_pair(at, i);
    bd[i] = make_pair(bt, i);
  }
  sort(ad.begin(), ad.end());
  sort(bd.begin(), bd.end());
  double sum = 1e18;
  {
    sum = min(sum, ad[0].first);
    sum = min(sum, bd[0].first);
    if (n >= 2) {
      double u = ad[0].first;
      double v = bd[0].first;
      int uidx = ad[0].second;
      int vidx = bd[0].second;
      if (uidx != vidx) {
	sum = min(sum, u + v);
      } else {
	double u2 = ad[1].first;
	double v2 = bd[1].first;
	sum = min(sum, u + v2);
	sum = min(sum, u2 + v);
      }
    }
  }
  REP(i, 0, n) {
    sum += 2 * d[i];
  }
  printf("%.15f\n", sum);
}
