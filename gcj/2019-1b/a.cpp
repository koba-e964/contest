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

int solve(int q, const vector<PI> &d) {
  vector<int> a(q + 2);
  REP(i, 0, d.size()) {
    PI u = d[i];
    if (u.second == 1) {
      a[u.first] += 1;
      a[q + 1] -= 1;
    } else {
      a[0] += 1;
      a[u.first + 1] -= 1;
    }
  }
  REP(i, 0, q + 1) {
    a[i + 1] += a[i];
  }
  PI mi(1e8, -1);
  REP(i, 0, q + 1) {
    mi = min(mi, PI(-a[i], i));
  }
  return mi.second;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int p, q;
    cin >> p >> q;
    vector<PI> cx, cy;
    REP(i, 0, p) {
      int x, y;
      char c;
      cin >> x >> y >> c;
      if (c == 'N') {
        cy.push_back(PI(y + 1, 1));
      }
      if (c == 'S') {
        cy.push_back(PI(y - 1, -1));
      }
      if (c == 'E') {
        cx.push_back(PI(x + 1, 1));
      }
      if (c == 'W') {
        cx.push_back(PI(x - 1, -1));
      }
    }
    cout << "Case #" << case_nr << ": "
         << solve(q, cx) << " " << solve(q, cy) << "\n";
  }
}
