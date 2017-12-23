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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;

struct cmp {
  bool operator()(PL x, PL y) const {
    if (x.first <= y.first && x.second <= y.second) {
      return x != y;
    }
    if (x.first >= y.first && x.second >= y.second) {
      return false;
    }
    bool flip = false;
    if (x > y) {
      swap(x, y);
      flip = true;
    }
    bool ans = false;
    // x.0 <= y.0, x.1 >= y.1
    if (x.first >= y.second) {
      ans = false;
    } else {
      ans = true;
    }
    if (flip) ans = !ans;
    return ans;
  }
};

const ll inf = 1e16;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL h(n), p(n);
  vector<PL> pool;
  vector<VL> dp(n + 1, VL(n + 1, inf));
  REP(i, 0, n) {
    cin >> h[i] >> p[i];
    pool.push_back(PL(h[i]+p[i], p[i]));
  }
  sort(pool.begin(), pool.end());
  dp[0][0] = 0;
  REP(i, 0, n) {
    ll y = pool[i].second;
    ll x = pool[i].first - y;
    REP(j, 0, i + 2) {
      dp[i + 1][j] = dp[i][j];
      if (j >= 1 && dp[i][j - 1] <= x) {
	dp[i + 1][j] = min(dp[i + 1][j], dp[i][j - 1] + y);
      }
    }
  }
  int ma = 0;
  REP(i, 0, n + 1) {
    if (dp[n][i] < inf) {
      ma = max(ma, i);
    }
  }
  cout << ma << "\n";
}
