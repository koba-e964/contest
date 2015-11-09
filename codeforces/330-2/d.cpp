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
typedef pair<int, int> PI;
const double EPS=1e-9;

double pi = abs(acos(-1));

double ff(double dist) {
  // solve t - sin(t) = pi + dist / 2
  double goal = dist / 2;
  double lo = 0, hi = pi;
  REP(i, 0, 80) {
    double mid = (lo + hi) / 2;
    if (mid + sin(mid) <= goal) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  return 2 * hi;
}

int main(void){
  int n;
  double r, v;
  cin >> n >> r >> v;
  REP(i, 0, n) {
    double s,f;
    cin >> s >> f;
    s = f - s; // f = 0;
    double rot = floor(s / (2 * pi * r));
    double rem = s - rot * 2 * pi * r;
    printf("%.10f\n", (rot * 2 * pi + ff(rem / r)) * r / v);
  }
}
