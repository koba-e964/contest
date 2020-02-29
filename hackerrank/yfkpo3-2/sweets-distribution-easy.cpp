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

const int N = 200001;

ll a[4][N];

ll dp[4][N];

const ll inf = 1e16;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, 4) {
    REP(j, 0, n) cin >> a[i][j + 1];
    REP(j, 0, n) a[i][j + 1] += a[i][j];
  }
  REP(i, 1, 4) REP(j, 0, N) dp[i][j] = -inf;
  REP(j, 0, N) dp[0][j] = 0;
  REP(i, 0, 3) {
    REP(j, 1, n + 1) {
      dp[i + 1][j] = dp[i][j - 1] + a[i][j] - a[i + 1][j];
    }
    REP(j, 1, n + 1) {
      dp[i + 1][j] = max(dp[i + 1][j], dp[i + 1][j - 1]);
    }
  }
  cout << dp[3][n - 1] + a[3][n] << endl;
}
