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

const int inf = 1e8;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  string a, b;
  cin >> n >> a >> b;
  VI dp(n + 1, inf);
  dp[0] = 0;
  REP(i, 0, n) {
    dp[i + 1] = dp[i] + (a[i] == b[i] ? 0 : 1);
    if (i >= 1 && a[i] != a[i - 1] && a[i] == b[i - 1] && a[i - 1] == b[i]) {
      dp[i + 1] = min(dp[i - 1] + 1, dp[i + 1]);
    }
  }
  cout << dp[n] << endl;
}
