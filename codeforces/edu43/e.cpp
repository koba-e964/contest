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
typedef pair<ll, int> PLI;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, a, b;
  cin >> n >> a >> b;
  b = min(b, n);
  VL h(n), d(n);
  REP(i, 0, n) cin >> h[i] >> d[i];
  vector<PLI> pool;
  REP(i, 0, n) {
    pool.push_back(PLI(max(0LL, h[i] - d[i]), i));
  }
  sort(pool.rbegin(), pool.rend());
  ll tot = 0;
  REP(i, 0, n) tot += d[i];
  ll ma = 0;
  ll delta_base = 0;
  if (b == 0) {
    cout << tot << endl;
    return 0;
  }
  REP(i, 0, b) delta_base += pool[i].first;
  if (0) {
    DEBUGP(tot);
    DEBUGP(delta_base);
  }
  REP(i, 0, n) {
    int idx = pool[i].second;
    ll nd = max(0LL, (h[idx] << a) - d[idx]);
    if (i < b) {
      ma = max(ma, delta_base + nd - pool[i].first);
    } else {
      ma = max(ma, delta_base + max(0LL, nd - pool[b - 1].first));
    }
  }
  cout << tot + ma << endl;
}
