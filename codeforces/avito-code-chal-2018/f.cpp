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

vector<PI> canon(int n, PI x) {
  if (x.first == x.second) return vector<PI>();
  if (x.second - x.first >= n) return vector<PI>(1, PI(0, n));
  int q = x.first / n;
  x.first -= q * n;
  x.second -= q * n;
  if (x.second <= n) return vector<PI>(1, x);
  vector<PI> ret(2);
  ret[1] = PI(x.first, n);
  ret[0] = PI(0, x.second - n);
  return ret;
}

vector<PI> inters(const vector<PI> &a, const vector<PI> &b) {
  int n = a.size();
  int m = b.size();
  vector<PI> ret;
  REP(i, 0, n) {
    REP(j, 0, m) {
      if (a[i].second <= b[j].first || b[j].second <= a[i].first) continue;
      ret.push_back(PI(max(a[i].first, b[j].first), min(a[i].second, b[j].second)));
    }
  }
  // merge
  vector<PI> tret;
  REP(i, 0, ret.size()) {
    if (i == 0 || ret[i].first != ret[i - 1].second) {
      tret.push_back(ret[i]);
    } else {
      tret.back().second = ret[i].second;
    }
  }
  REP(i, 0, tret.size()) assert (tret[i].first < tret[i].second);
  return tret;
}

bool ok(ll mid, int n, const VL &a, const VL &b) {
  vector<PI> hamko(1, PI(0, n));
  REP(i, 0, n) {
    // [st, en)
    int st = lower_bound(b.begin(), b.end(), a[i + n] - mid) - b.begin();
    int en = upper_bound(b.begin(), b.end(), a[i + n] + mid) - b.begin();
    st -= i;
    en -= i;
    vector<PI> sub = canon(n, PI(st, en));
    hamko = inters(hamko, sub);
  }
  if (DEBUG) {
    DEBUGP(mid);
    REP(i, 0, hamko.size()) {
      cerr << "[" << hamko[i].first << "," << hamko[i].second << ")"<<endl;
    }
    REP(i, 0, hamko.size() - 1) {
      assert (hamko[i].second < hamko[i + 1].first);
    }
  }
  return hamko.size() > 0;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll l;
  cin >> n >> l;
  VL a(3 * n), b(3 * n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) cin >> b[i];
  sort(a.begin(), a.begin() + n);
  sort(b.begin(), b.begin() + n);
  REP(j, 1, 3) {
    REP(i, 0, n) a[i + j * n] = a[i] + j * l;
    REP(i, 0, n) b[i + j * n] = b[i] + j * l;
  }
  ll pass = l / 2, fail = -1;
  while (pass - fail > 1) {
    if (DEBUG) {
      REP(i, 0, 100) {
        ok(i, n, a, b);
      }
    }
    ll mid = (pass + fail) / 2;
    if (ok(mid, n, a, b)) pass = mid;
    else fail = mid;
  }
  cout << pass << endl;
}
