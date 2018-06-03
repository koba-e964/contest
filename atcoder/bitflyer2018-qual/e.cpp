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

map<ll, int> tbl;
ll intv = 0;
ll sz = 0;
ll d;
void add(ll day) {
  if (tbl.count(day) != 0 && tbl[day] > 0) {
    tbl[day]++;
    return;
  }
  auto miit = tbl.upper_bound(day - 1);
  auto mait = tbl.lower_bound(day + 1);
  ll mi = -1e18;
  ll ma = 1e18;
  if (miit != tbl.begin()) {
    miit--;
    mi = miit->first;
  }
  if (mait != tbl.end()) {
    ma = mait->first;
  }
  tbl[day] = 1;
  sz++;
  assert (mi < day);
  assert (ma > day);
  if (DEBUG) {
    DEBUGP(day);
    DEBUGP(mi);
    DEBUGP(ma);
  }
  if (ma - mi - 1 <= d) {
    intv--;
    return;
  }
  if (day - mi - 1 <= d) intv += day - mi - 1;
  if (ma - day - 1 <= d) intv += ma - day - 1;
}

void erase(ll day) {
  assert (tbl.count(day) > 0);
  if (tbl[day] >= 2) {
    tbl[day]--;
    return;
  }
  auto miit = tbl.upper_bound(day - 1);
  auto mait = tbl.lower_bound(day + 1);
  ll mi = -1e18;
  ll ma = 1e18;
  if (miit != tbl.begin()) {
    miit--;
    mi = miit->first;
  }
  if (mait != tbl.end() && mait->first > day) {
    ma = mait->first;
  }
  tbl[day] = 0;
  tbl.erase(day);
  sz--;
  assert (mi < day);
  assert (ma > day);
  if (DEBUG) {
    DEBUGP(-day);
    DEBUGP(mi);
    DEBUGP(ma);
  }
  if (ma - mi - 1 <= d) {
    intv++;
    return;
  }
  if (day - mi - 1 <= d) intv -= day - mi - 1;
  if (ma - day - 1 <= d) intv -= ma - day - 1;
}

void debug(void) {
  if (DEBUG) {
    cerr << "data:" << endl;
    DEBUGP(sz);
    DEBUGP(intv);
    for (auto e: tbl) {
      if (e.second > 0) {
        cerr << e.first << " " << e.second << endl;
      }
    }
  }
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll y, w, n, m;
  cin >> y >> w >> n >> m >> d;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
  }
  VL b(m), c(m);
  vector<VL> sigma(w);
  REP(i, 0, m) {
    cin >> b[i] >> c[i];
    b[i]--;
    c[i]--;
    sigma[c[i]].push_back(b[i]);
  }
  // insert all
  REP(i, 0, n) add(a[i]);
  REP(i, 0, m) {
    ll day = b[i] * w + c[i];
    add(day);
  }
  REP(i, 0, w) {
    if (DEBUG) DEBUGP(i);
    debug();
    cout << sz + intv << endl;
    // TODO a
    REP(j, 0, n) {
      erase(a[j] + i);
      add(a[j] + i + 1);
    }
    for (ll b: sigma[i]) {
      erase(b * w + i);
      add(b * w + i + w);
    }
  }
}
