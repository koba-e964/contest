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



int main(void){
  int n;
  cin >> n;
  vector<double> x(n), v(n);
  REP(i, 0, n) {
    cin >> x[i];
  }
  REP(i, 0, n) {
    cin >> v[i];
  }
  double lo = 0.0;
  double hi = 1e9;
  REP(iter_cnt, 0, 60) {
    double mid = (hi + lo) / 2;
    // check
    double sup = 1.0 / 0.0;
    double inf = -1.0 / 0.0;
    REP(i, 0, n) {
      sup = min(sup, x[i] + v[i] * mid);
      inf = max(inf, x[i] - v[i] * mid);
    }
    if (sup >= inf) {
      hi = mid;
    } else {
      lo = mid;
    }
  }
  printf("%.15f\n", hi);
}
