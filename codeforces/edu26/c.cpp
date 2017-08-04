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

bool fits(int a, int b, int x1, int y1, int x2, int y2) {
  int x = x1 + x2;
  int y = y1 + y2;
  return (x <= a && max(y1, y2) <= b) || (max(y1, y2) <= a && x <= b)
    || (y <= a && max(x1, x2) <= b) || (max(x1, x2) <= a && y <= b);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, a, b;
  cin >> n >> a >> b;
  VI x(n), y(n);
  REP(i, 0, n) {
    cin >> x[i] >> y[i];
  }
  int ma = 0;
  REP(i, 0, n) {
    REP(j, 0, i) {
      if (fits(a, b, x[i], y[i], x[j], y[j])
	  || fits(a, b, x[i], y[i], y[j], x[j])) {
	ma = max(ma, x[i] * y[i] + x[j] * y[j]);
      }
    }
  }
  cout << ma << "\n";
}
