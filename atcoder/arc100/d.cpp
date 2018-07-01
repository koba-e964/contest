#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const ll inf = 1e16;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  VL acc(n + 1);
  REP(i, 0, n) acc[i + 1] = acc[i] + 2 * a[i];
  VL tap(n + 1, inf), ris(n + 1, inf);
  ll glmi = inf;
  REP(i, 1, n) {
    ll ma = -inf, mi = inf;
    ll sum = acc[i] / 2;
    int idx = lower_bound(acc.begin(), acc.begin() + i, sum) - acc.begin();
    REP(delta, -2, 4) {
      int u = idx + delta;
      if (u > 0 && u < i) {
        tap[i] = min(tap[i], abs(acc[u] - sum));
      }
    }
    ma = max(ma, sum + tap[i]);
    mi = min(mi, sum - tap[i]);
    sum = (acc[n] - acc[i]) / 2;
    idx = lower_bound(acc.begin() + i, acc.end(), sum + acc[i]) - acc.begin();
    REP(delta, -2, 4) {
      int u = idx + delta;
      if (u > i && u < n) {
        ris[i] = min(ris[i], abs(acc[u] + sum - acc[n]));
      }
    }
    ma = max(ma, sum + ris[i]);
    mi = min(mi, sum - ris[i]);
    glmi = min(glmi, ma - mi);
  }
  cout << glmi / 2 << endl;
}
