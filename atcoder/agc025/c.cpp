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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


const int N = 200010;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI l(n), r(n);
  REP(i, 0, n) {
    cin >> l[i] >> r[i];
  }
  sort(l.rbegin(), l.rend());
  sort(r.begin(), r.end());
  VL acc_l(n + 1), acc_r(n + 1);
  REP(i, 0, n) acc_l[i + 1] = acc_l[i] + l[i];
  REP(i, 0, n) acc_r[i + 1] = acc_r[i] + r[i];
  ll tot = 0;
  REP(x, 0, n + 1) {
    REP(delta, -1, 2) {
      int y = x + delta;
      if (y < 0 || y > n) continue;
      if (DEBUG) {
        DEBUGP(x);
        DEBUGP(y);
        DEBUGP(acc_l[x]);
        DEBUGP(acc_r[y]);
      }
      tot = max(tot, 2 * (acc_l[x] - acc_r[y]));
    }
  }
  cout << tot << endl;
}
