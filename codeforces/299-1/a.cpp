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

ll solve(ll a, ll b, ll t, ll m) {
  if (a > t) {
    return -1;
  }
  ll lo = 0;
  ll hi = (t - a) / b + 2;
  while (hi - lo >= 2) {
    ll mid = (lo + hi) / 2;
    bool ok = mid * (mid - 1) / 2 * b + a * mid <= m * t;
    if (mid <= m) {
      ok &= a + (mid - 1) * b <= t;
    }
    if (ok) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  return lo;
}

int main(void){
  int a,b,n;
  cin >> a >> b >> n;
  REP(loop_var, 0, n) {
    ll l, t, m;
    cin >> l >> t >> m;
    int res = solve((l - 1) * b + a, b, t, m);
    cout << (res == -1 ? -1 : res + l - 1) << endl;
  }
}
