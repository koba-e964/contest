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
const ll mod = 1e9 + 7;

const int W = 201;

int main(void) {
  int n;
  cin >> n;
  VI t(n), v(n);
  int tot = 0;
  REP(i, 0, n) {
    cin >> t[i];
    t[i] *= 2;
    tot += t[i];
  }
  VI limit(tot);
  int cur = 0;
  REP(i, 0, n) {
    cin >> v[i];
    v[i] *= 2;
    REP(j, cur, cur + t[i]) {
      limit[j] = v[i];
    }
    cur += t[i];
  }
  vector<VL> dp(tot + 1, VL(W, -1));
  dp[0][0] = 0;
  REP(i, 0, tot) {
    REP(j, 0, limit[i] + 1) {
      if (dp[i][j] >= 0)
	dp[i + 1][j] = max(dp[i + 1][j], dp[i][j] + 2 * j);
      if (j >= 1) {
	if (dp[i][j - 1] >= 0)
	  dp[i + 1][j] = max(dp[i + 1][j], dp[i][j - 1] + 2 * j - 1);
      }
      if (j < limit[i]) {
	if (dp[i][j + 1] >= 0)
	  dp[i + 1][j] = max(dp[i + 1][j], dp[i][j + 1] + 2 * j + 1);
      }
    }
  }
  cout << setprecision(17) << dp[tot][0] / 8.0 << "\n";
}
