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

const int DEBUG = 0;

bool check(int a, int b, int c, int d) {
  if (a > b) swap(a, b);
  if (c > d) swap(c, d);
  if (a < c && b < d) {
    return true;
  }
  if (a > c) {
    return 0;
  }
  assert (a < c && b >= d);
  assert (b > c);
  /* exists deg. a * cos(deg) + b * sin(deg) < c &&
   * a * sin(deg) + b * cos(deg) < d ?
   */
  double lo = 0;
  double hi = acos(-1) / 2;
  REP(loop_cnt, 0, 100) {
    double mid = (lo + hi) / 2;
    if (a * cos(mid) + b * sin(mid) < c) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  if (DEBUG) {
    cout << "lo=" << lo << ", hi=" << hi << endl;
  }
  return a * sin(lo) + b * cos(lo) < d;
}

int main(void){
  int a, b, n;
  cin >> a >> b >> n;
  REP(i, 0, n) {
    int c, d;
    cin >> c >> d;
    cout << (check(a, b, c, d) ? "YES" : "NO") << endl;
  }
}
