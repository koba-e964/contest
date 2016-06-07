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



int main(void){
  int n;
  ll h, a, b, c, d, e;
  cin >> n >> h >> a >> b >> c >> d >> e;

  // Discard e
  h = n * e - h;
  b += e;
  d += e;

  // Goal: min {a * x + c * y | b * x + d * y >= h, x + y <= n, x, y >= 0 }



  ll mi = 1e16;
  for (ll x = 0; x <= n; ++x) {
    ll y = (h - b * x + d) / d;
    if (y < 0) y = 0;
    if (y <= n - x) {
      mi = min(mi, a * x + c * y);
    }
  }

  cout << mi << endl;
  
}
