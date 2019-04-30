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

const int N = 400;
ll dp[N][N];
const ll inf = 1e17;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, k;
  cin >> n >> m >> k;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n + 1) REP(j, 0, m + 1) dp[i][j] = -inf;
  dp[0][0] = 0;
  REP(i, 0, n) {
    REP(j, 1, min(k, i + 1) + 1) {
      REP(l, 0, m) {
        dp[i + 1][l + 1] = max(dp[i + 1][l + 1], dp[i - j + 1][l] + a[i]);
      }
    }
  }
  ll ma = -inf;
  REP(i, n - k + 1, n + 1) ma = max(ma, dp[i][m]);
  cout << (ma < 0 ? -1 : ma) << endl;
  
}
