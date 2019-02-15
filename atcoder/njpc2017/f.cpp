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

// This solution is made after the author read the editorial.
bool ok(int n, const vector<double> &t, const vector<double> &x, double v) {
  vector<pair<double, double> > dp(n + 1);
  const double INF = 1e18;
  dp[0] = make_pair(0.0, 0.0);
  REP(i, 0, n) {
    double dist = v * (t[i + 1] - t[i]);
    dp[i + 1] = make_pair(INF, -INF);
    if (abs(x[i + 1] - x[i]) <= dist) {
      dp[i + 1].first = min(dp[i + 1].first, dp[i].first - dist);
      dp[i + 1].second = max(dp[i + 1].second, dp[i].second + dist);
    }
    if (dp[i].first - dist <= x[i + 1] && x[i + 1] <= dp[i].second + dist) {
      dp[i + 1].first = min(dp[i + 1].first, x[i] - dist);
      dp[i + 1].second = max(dp[i + 1].second, x[i] + dist);
    }
    if (dp[i + 1].first > dp[i + 1].second) return false;
  }
  return true;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  vector<double> t(n + 1), x(n + 1);
  REP(i, 0, n) cin >> t[i + 1] >> x[i + 1];
  double pass = 1e7, fail = 0;
  REP(i, 0, 100) {
    double mid = (pass + fail) / 2;
    if (ok(n, t, x, mid)) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  cout << setprecision(15) << pass << endl;
}
