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

ll calc(const VL &means, ll x1, ll y1, ll x2, ll y2, ll v) {
  if (x1 == x2) return abs(y1 - y2);
  if (y1 > y2) swap(y1, y2);
  ll mi = 1e15;
  // elevator inside?
  ll vert = (abs(x1 - x2) + v - 1) / v;
  auto st = lower_bound(means.begin(), means.end(), y1);
  auto en = upper_bound(means.begin(), means.end(), y2);
  if (st < en) {
    mi = min(mi, abs(y1 - y2) + vert);
  }
  if (en != means.end()) {
    ll y = *en;
    mi = min(mi, y - y2 + y - y1 + vert);
  }
  if (st != means.begin()) {
    st--;
    ll y = *st;
    mi = min(mi, -(y - y2 + y - y1) + vert);
  }
  // cerr << "v = " << v << " " << mi << endl;
  return mi;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, cl, ce;
  ll v;
  cin >> n >> m >> cl >> ce >> v;
  VL l(cl);
  REP(i, 0, cl) cin >> l[i];
  VL e(ce);
  REP(i, 0, ce) cin >> e[i];
  int q;
  cin >> q;
  REP(i, 0, q) {
    ll x1, y1, x2, y2;
    cin >> x1 >> y1 >> x2 >> y2;
    ll mi = 1e15;
    mi = min(mi, calc(e, x1, y1, x2, y2, v));
    mi = min(mi, calc(l, x1, y1, x2, y2, 1));
    // elevator inside?
    cout << mi << "\n";
  }
}
