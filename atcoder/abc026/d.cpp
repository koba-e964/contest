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

double a,b,c;

double f(double t) {
  double aarg = c * t;
  aarg -= floor(aarg / 2) * 2;
  return a * t + b * sin(aarg * acos(-1));
}
double f_cor(double t) {
  double aarg = c * t;
  return a * t + b * sin(aarg * acos(-1));
}

int main(void){
  cin >> a >> b >> c;
  double lo = 0.0;
  double hi = 100;
  assert (f(hi) >= 100);
  while (abs(f(lo) - 100) >= 1e-7) {
    bool res = f(lo) > f(hi);
    double mid = (lo + hi) / 2;
    if (f(mid) >= 100) {
      if (res) lo = mid; else hi = mid;
    } else {
      if (res) hi = mid; else lo = mid;
    }
  }
  printf("%.15f\n", lo);
}
