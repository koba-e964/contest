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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll c;
  cin >> n >> c;
  VL x(n), v(n);
  REP(i, 0, n) cin >> x[i] >> v[i];
  VL lef(n + 1), rig(n + 1);
  REP(i, 0, n) lef[i + 1] = lef[i] + v[i];
  for (int i = n - 1; i >= 0; --i) rig[i] = rig[i + 1] + v[i];
  ll ma = 0;
  VL lef_ma(n + 1), rig_ma(n + 1);
  REP(i, 0, n) lef_ma[i + 1] = max(lef_ma[i], lef[i + 1] - x[i]);
  for (int i = n - 1; i >= 0; --i)
    rig_ma[i] = max(rig_ma[i + 1], rig[i] - c + x[i]);
  ma = max(ma, max(rig_ma[0], lef_ma[n]));
  if (0) {
    REP(i, 0, n + 1) DEBUGP(lef[i]);
    REP(i, 0, n + 1) DEBUGP(rig[i]);
    REP(i, 0, n + 1) DEBUGP(lef_ma[i]);
    REP(i, 0, n + 1) DEBUGP(rig_ma[i]);
  }
  REP(i, 0, n) {
    ma = max(ma, lef[i + 1] - 2 * x[i] + rig_ma[i + 1]);
    ma = max(ma, rig[i] - 2 * (c - x[i]) + lef_ma[i]);
  }
  cout << ma << endl;
}
