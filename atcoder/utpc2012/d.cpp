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

typedef complex<double> comp;

int main(void){
  int n;
  cin >> n;
  vector<comp> pt(n), pt2(n);
  REP(i, 0, 2 * n) {
    double x, y;
    cin >> x >> y;
    comp t(x, y);
    if (i < n) {
      pt[i] = t;
    } else {
      pt2[i - n] = t;
    }
  }
  // pt2[0] = a * pt[0] + b
  // pt2[1] = a * pt[1] + b
  comp a = (pt2[0] - pt2[1]) / (pt[0] - pt[1]);
  comp b = pt2[0] - a * pt[0];
  // x = a * x + b
  comp x = b / (1.0 - a);
  printf("%.15f %.15f\n", x.real(), x.imag());
}
