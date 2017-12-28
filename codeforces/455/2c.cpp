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
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n);
  REP(i, 0, n) {
    string ty;
    cin >> ty;
    a[i] = ty == "f" ? 1 : 0;
  }
  vector<VL> dp(n + 1, VL(n + 1, 0));
  dp[0][0] = 1;
  REP(i, 0, n) {
    if (i == 0 || a[i - 1] == 0) {
      // only level k <= j is allowed
      REP(j, 0, n + 1) {
	dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % mod;
      }
      // cumulative
      for(int j = n - 1; j >= 0; --j) {
	dp[i + 1][j] = (dp[i + 1][j] + dp[i + 1][j + 1]) % mod;
      }
    } else {
      // prev is for statement
      REP(j, 0, n) {
	dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j]) % mod;
      }
    }
  }
  ll tot = 0;
  REP(i, 0, n + 1) {
    tot = (tot + dp[n][i]) % mod;
  }
  cout << tot << "\n";
}
