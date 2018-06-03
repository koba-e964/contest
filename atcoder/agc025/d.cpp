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
const ll mod = 1e9 + 7;

PI sqr(int e) {
  int k = 1;
  while (1) {
    if (k * k >= e) {
      break;
    }
    k *= 2;
  }
  if (k * k == e) return PI(k, 0);
  return PI(k / 2, 1);
}

bool filter(int x, int y, PI t) {
  if (t.second == 0) {
    int p1 = x / t.first, p2 = y / t.first;
    return (p1 + p2) % 2 == 0;
  }
  return (y / t.first) % 2 == 0;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, d1, d2;
  cin >> n >> d1 >> d2;
  int e1 = d1 & -d1;
  int e2 = d2 & -d2;
  if (e1 < e2) swap(e1, e2);
  PI t1 = sqr(e1);
  PI t2 = sqr(e2);
  vector<PI> ans;
  REP(i, 0, 2 * n) {
    REP(j, 0, 2 * n) {
      if (filter(i, j, t1) && filter(i, j, t2)) {
        ans.push_back(PI(i, j));
      }
    }
  }
  assert ((int) ans.size() >= n * n);
  REP(i, 0, n * n) {
    cout << ans[i].first << " " << ans[i].second << endl;
  }
  if (DEBUG) {
    REP(i, 0, ans.size()) {
      REP(j, 0, ans.size()) {
        ll x = (ans[i].first - ans[j].first);
        ll y = (ans[i].second - ans[j].second);
        ll d = x * x + y * y;
        assert (d != d1);
        assert (d != d2);
      }
    }
  }
}
