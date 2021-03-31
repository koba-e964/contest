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

const ll inf = 1e18;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int x, y;
    string s;
    cin >> x >> y >> s;
    int n = s.size();
    vector<VL> dp(n, VL(2, inf));
    if (s[0] != 'J') dp[0][0] = 0;
    if (s[0] != 'C') dp[0][1] = 0;
    REP(i, 1, n) {
      if (s[i] != 'J') {
        dp[i][0] = min(dp[i][0], dp[i - 1][0]);
        dp[i][0] = min(dp[i][0], dp[i - 1][1] + y);
      }
      if (s[i] != 'C') {
        dp[i][1] = min(dp[i][1], dp[i - 1][1]);
        dp[i][1] = min(dp[i][1], dp[i - 1][0] + x);
      }
    }
    ll ans = min(dp[n - 1][0], dp[n - 1][1]);
    cout << "Case #" << case_nr << ": " << ans << "\n";
  }
}
