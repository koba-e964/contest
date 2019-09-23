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

const int N = 400000;
const int B = 200000;
int dp[101][N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  REP(i, 0, 101) {
    REP(j, 0, N) {
      dp[i][j] = inf;
    }
  }
  dp[0][2019 + B] = 0;
  REP(i, 0, n) {
    int a;
    string b;
    cin >> a >> b;
    if (b == "P") a = -a;
    REP(j, 0, N) dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
    REP(j, 0, N) {
      int to = j + a;
      if (to < 0 || to >= N) continue;
      dp[i + 1][to] = min(dp[i + 1][to], dp[i][j] + 1);
    }
  }
  int ans = dp[n][k + B];
  cout << (ans >= inf ? -1 : ans) << endl;
}
