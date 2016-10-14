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
  double d;
  int case_cnt = 0;
  while (scanf("%d%lf", &n, &d)) {
    if (n == 0 && d == 0) {
      return 0;
    }
    typedef pair<double, double> PD;
    vector<PD> v(n);
    bool ok = 1;
    REP(i, 0, n) {
      double x, y;
      scanf("%lf%lf", &x, &y);
      if (y > d) {
	ok = 0;
      } else {
	double dist = sqrt(d * d - pow(y, 2));
	v[i] = PD(x + dist, x - dist);
      }
    }
    int cnt = 0;
    if (!ok) {
      cnt = -1;
    } else {
      sort(v.begin(), v.end());
      double cur = -1.0 / 0.0;
      int pos = 0;
      while (pos < n) {
	if (cur < v[pos].second - 1e-9) {
	  cur = v[pos].first;
	  cnt++;
	}
	pos++;
      }
    }
    case_cnt++;
    printf("Case %d: %d\n", case_cnt, cnt);
  }
  
}
